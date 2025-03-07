//! Application state and logic
//!
//! This module contains the application state and logic for the TUI IDE.

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use anyhow::{Context, Result};
use tui::widgets::ListState;
use tui::style::Color;

use crate::tui::vscode::status_bar::StatusItem;
use crate::tui::syntax::SyntaxHighlighter;
use crate::tui::git::GitManager;
use crate::tui::vscode::command_palette::Command;
use crate::lessvm::VM;
use super::widgets::{CodeViewState, MemoryViewState}; 

pub use super::vscode::panel_area::PanelTab;
/// Result type used within the application
pub type AppResult<T> = std::result::Result<T, anyhow::Error>;

/// Tab enum for representing different tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    /// Code editor tab
    Editor = 0,
    /// Disassembler tab
    Disassembler = 1,
    /// VM state tab (registers, flags)
    VMState = 2,
    /// Memory viewer tab
    Memory = 3,
    /// Console/output tab
    Console = 4,
}

/// Application mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    /// Normal mode for navigation
    Normal,
    /// Edit mode for editing text
    Edit,
    /// Command mode for entering commands
    Command,
    /// Dialog mode for interacting with dialogs
    Dialog,
    /// Help mode for viewing help
    Help,
}

/// VM status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VMStatus {
    /// VM is stopped
    Stopped,
    /// VM is running
    Running,
    /// VM is paused
    Paused,
}

/// Dialog types
// We implement Debug and Clone manually below
pub enum DialogType {
    /// Confirmation dialog with a message and callback
    Confirm(String, Box<dyn Fn(&mut App) + Send + Sync>),
    /// Input dialog with a title, input value, and callback
    Input(String, String, Box<dyn Fn(&mut App, &str) + Send + Sync>),
    /// Message dialog with a message
    Message(String),
}

// Manual implementation of Debug for DialogType
impl std::fmt::Debug for DialogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogType::Confirm(msg, _) => f.debug_tuple("Confirm").field(msg).field(&"<callback>").finish(),
            DialogType::Input(title, value, _) => f.debug_tuple("Input").field(title).field(value).field(&"<callback>").finish(),
            DialogType::Message(msg) => f.debug_tuple("Message").field(msg).finish(),
        }
    }
}

// Manual implementation of Clone for DialogType
impl Clone for DialogType {
    fn clone(&self) -> Self {
        match self {
            DialogType::Confirm(msg, _) => {
                // We can't clone the callback, so we provide a no-op one
                DialogType::Confirm(msg.clone(), Box::new(|_| {}))
            },
            DialogType::Input(title, value, _) => {
                // We can't clone the callback, so we provide a no-op one
                DialogType::Input(title.clone(), value.clone(), Box::new(|_, _| {}))
            },
            DialogType::Message(msg) => DialogType::Message(msg.clone()),
        }
    }
}

/// Sidebar view types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarView {
    /// Explorer view
    Explorer,
    /// Search view
    Search,
    /// Source Control view
    SourceControl,
    /// Debug view
    Debug,
    /// Extensions view
    Extensions,
}

/// Activity Bar item
#[derive(Debug, Clone)]
pub struct ActivityBarItem {
    /// Item ID
    pub id: String,
    /// Item label (for tooltips)
    pub label: String,
    /// Icon representation (using unicode)
    pub icon: String,
    /// Associated sidebar view
    pub view: SidebarView,
}

/// Sidebar file entry
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// File name
    pub name: String,
    /// Full path
    pub path: PathBuf,
    /// Whether this is a directory
    pub is_dir: bool,
    /// Child entries (if directory)
    pub children: Option<Vec<FileEntry>>,
}

/// File node for explorer
#[derive(Debug, Clone)]
pub struct FileNode {
    /// Path
    pub path: String,
    /// Whether this is a directory
    pub is_dir: bool,
    /// Whether this is expanded (for directories)
    pub expanded: bool,
    /// Children (for directories)
    pub children: Vec<FileNode>,
    /// Git status (if any)
    pub git_status: Option<String>,
}

/// Problem information
#[derive(Debug, Clone)]
pub struct Problem {
    /// File path
    pub file: String,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
    /// Problem message
    pub message: String,
    /// Severity (error, warning, info)
    pub severity: String,
}

/// Main application state
pub struct App {
    /// Whether the application is running
    pub running: bool,
    
    /// Current tab
    pub current_tab: Tab,
    
    /// Current mode
    pub mode: AppMode,
    
    /// Current file being edited
    pub current_file: Option<PathBuf>,
    
    /// Contents of the current file
    pub file_contents: Vec<String>,
    
    /// Whether the file has been modified
    pub modified: bool,
    
    /// Code view state
    pub code_view_state: CodeViewState,
    
    /// Memory view state
    pub memory_view_state: MemoryViewState,
    
    /// VM instance
    pub vm: VM,
    
    /// VM status
    pub vm_status: VMStatus,
    
    /// Disassembly of the loaded program
    pub disassembly: Vec<String>,
    
    /// Console output history
    pub console_history: Vec<String>,
    
    /// Console list state for UI
    pub console_list_state: ListState,
    
    /// Command input
    pub command_input: String,
    
    /// Command history
    pub command_history: Vec<String>,
    
    /// Command history index
    pub command_history_index: usize,
    
    /// Breakpoints (line number -> enabled)
    pub breakpoints: HashMap<usize, bool>,
    
    /// Memory watchpoints (address -> enabled)
    pub watchpoints: HashMap<usize, bool>,
    
    /// Memory snapshot for comparison
    pub memory_snapshot: Option<Vec<u8>>,
    
    /// VM refresh timer
    pub vm_refresh_timer: Instant,
    
    /// Active dialog
    pub dialog: Option<DialogType>,
    
    /// Last error
    pub last_error: Option<String>,

    /// Use VS Code-like UI
    pub use_vscode_ui: bool,

    //=========================================================================
    // VSCODE-LIKE UI STATE
    //=========================================================================
    // VS Code UI state
    
    /// Whether the sidebar is visible
    pub sidebar_visible: bool,
    
    /// Current sidebar view
    pub sidebar_view: SidebarView,
    
    /// Whether the panel is visible
    pub panel_visible: bool,
    
    /// Command palette visibility
    pub command_palette_visible: bool,
    
    /// Command palette input
    pub command_palette_input: String,
    
    /// Command palette items (filtered commands)
    pub command_palette_items: Vec<Command>,
    
    /// Command palette selected index
    pub palette_selected_index: Option<usize>,
    
    /// Command palette selection
    pub command_palette_selection: Option<usize>,
    
    /// Available commands
    pub commands: Vec<Command>,
    
    /// Status bar left items
    pub status_left_items: Vec<StatusItem>,
    
    /// Status bar middle items
    pub status_middle_items: Vec<StatusItem>,
    
    /// Status bar right items
    pub status_right_items: Vec<StatusItem>,
    
    /// Git branch
    pub git_branch: Option<String>,
    
    /// File explorer entries
    pub file_explorer: Vec<FileEntry>,

    /// Current active view in activity bar
    pub active_view: Option<crate::tui::vscode::activity_bar::ActivityView>,

    /// Current workspace path
    pub current_workspace: Option<PathBuf>,
    
    /// Current workspace name
    pub current_workspace_name: Option<String>,
    
    /// File explorer nodes
    pub file_explorer_nodes: Vec<FileNode>,
    
    /// Selected file index in explorer
    pub selected_file_idx: Option<usize>,
    
    /// Open files in editor
    pub open_files: Vec<FileEntry>,
    
    /// Active tab index in editor
    pub active_tab_index: usize,
    
    /// Editor scroll offset (vertical)
    pub editor_scroll_offset: usize,
    
    /// Currently selected panel tab
    pub selected_panel_tab: PanelTab,
    
    /// Terminal output history
    pub terminal_output: Vec<String>,
    
    /// Terminal input
    pub terminal_input: String,
    
    /// Terminal input active
    pub terminal_input_active: bool,
    
    /// Output content
    pub output_content: Vec<String>,
    
    /// Problems list
    pub problems: Vec<Problem>,
    
    /// Debug console output
    pub debug_console_output: Vec<String>,
    
    /// Debug console input
    pub debug_console_input: String,
    
    /// Debug console input active
    pub debug_console_input_active: bool,
    
    /// Custom panel tabs
    pub custom_panel_tabs: Vec<String>,
    
    /// Custom panel contents
    pub custom_panel_contents: Vec<Vec<String>>,
    
    /// Syntax highlighter
    pub syntax_highlighter: SyntaxHighlighter,
    
    /// Git integration
    pub git_manager: GitManager,
}

impl App {
    /// Create a new application
    pub fn new() -> Self {
        Self {
            running: true,
            current_tab: Tab::Editor,
            mode: AppMode::Normal,
            current_file: None,
            file_contents: Vec::new(),
            modified: false,
            code_view_state: CodeViewState::new(),
            memory_view_state: MemoryViewState::new(),
            vm: VM::new(),
            vm_status: VMStatus::Stopped,
            disassembly: Vec::new(),
            console_history: Vec::new(),
            console_list_state: ListState::default(),
            command_input: String::new(),
            command_history: Vec::new(),
            command_history_index: 0,
            breakpoints: HashMap::new(),
            watchpoints: HashMap::new(),
            memory_snapshot: None,
            vm_refresh_timer: Instant::now(),
            dialog: None,
            last_error: None,
            
            // VS Code UI state
            use_vscode_ui: true,  // Enable VS Code UI by default
            
            // Sidebar and panel visibility
            sidebar_visible: true,
            sidebar_view: SidebarView::Explorer,
            panel_visible: false,
            command_palette_visible: false,
            command_palette_input: String::new(),
            command_palette_items: Vec::new(),
            palette_selected_index: Some(0),
            command_palette_selection: Some(0),
            commands: vec![
                Command::new("file.new", "New File", "File").keybinding("Ctrl+N"),
                Command::new("file.open", "Open...", "File").keybinding("Ctrl+O"),
                Command::new("file.save", "Save", "File").keybinding("Ctrl+S"),
                Command::new("edit.undo", "Undo", "Edit").keybinding("Ctrl+Z"),
                Command::new("edit.redo", "Redo", "Edit").keybinding("Ctrl+Y"),
                Command::new("edit.cut", "Cut", "Edit").keybinding("Ctrl+X"),
                Command::new("edit.copy", "Copy", "Edit").keybinding("Ctrl+C"),
                Command::new("edit.paste", "Paste", "Edit").keybinding("Ctrl+V"),
                Command::new("edit.find", "Find", "Edit").keybinding("Ctrl+F"),
                Command::new("edit.replace", "Replace", "Edit").keybinding("Ctrl+H"),
                Command::new("view.toggleSidebar", "Toggle Sidebar", "View").keybinding("Ctrl+B"),
                Command::new("view.togglePanel", "Toggle Panel", "View").keybinding("Ctrl+J"),
                Command::new("debug.start", "Start Debugging", "Debug").keybinding("F5"),
                Command::new("debug.pause", "Pause", "Debug"),
                Command::new("debug.stop", "Stop Debugging", "Debug").keybinding("Shift+F5"),
                Command::new("debug.stepOver", "Step Over", "Debug").keybinding("F10"),
                Command::new("debug.stepInto", "Step Into", "Debug").keybinding("F11"),
            ],
            status_left_items: Vec::new(),
            status_middle_items: Vec::new(),
            status_right_items: Vec::new(),
            git_branch: None,
            file_explorer: Vec::new(),

            // Activity bar state
            active_view: Some(crate::tui::vscode::activity_bar::ActivityView::Explorer),

            // Workspace and file explorer
            current_workspace: None,
            current_workspace_name: None,
            file_explorer_nodes: Vec::new(),
            selected_file_idx: None,

            // Editor state
            open_files: Vec::new(),
            active_tab_index: 0,
            editor_scroll_offset: 0,

            // Panel state
            selected_panel_tab: PanelTab::Terminal,
            terminal_output: vec![
                "Welcome to ideless terminal".to_string(),
                "Type commands here to interact with the system".to_string(),
                "$ ".to_string(),
            ],
            terminal_input: String::new(),
            terminal_input_active: false,
            output_content: vec![
                "Build output will appear here".to_string(),
            ],
            problems: Vec::new(),
            debug_console_output: vec![
                "Debug console initialized".to_string(),
                "Type expressions to evaluate".to_string(),
            ],
            debug_console_input: String::new(),
            debug_console_input_active: false,
            custom_panel_tabs: Vec::new(),
            custom_panel_contents: Vec::new(),
            
            // Syntax highlighter
            syntax_highlighter: SyntaxHighlighter::new(),
            
            // Git integration
            git_manager: GitManager::new(),
        }
    }

    /// Toggle between classic and VS Code-like UI
    pub fn toggle_ui_style(&mut self) {
        self.use_vscode_ui = !self.use_vscode_ui;
        self.add_to_console(format!("Switched to {} UI", if self.use_vscode_ui { "VS Code-like" } else { "Classic" }));
    }

    /// Initialize the VS Code UI components
    pub fn init_vscode_ui(&mut self) {
        // Set up default status bar items
        self.status_left_items = vec![
            StatusItem::new("ideless", "ideless".to_string()).fg(Color::White).bg(Color::DarkGray)
        ];

        // Initialize default file explorer with some sample files
        if self.file_explorer.is_empty() {
            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            
            // Sample directory structure - would be replaced with actual filesystem in real app
            self.file_explorer = vec![
                FileEntry {
                    name: "src".to_string(),
                    path: current_dir.join("src"),
                    is_dir: true,
                    children: Some(vec![
                        FileEntry {
                            name: "main.rs".to_string(),
                            path: current_dir.join("src/main.rs"),
                            is_dir: false,
                            children: None,
                        },
                        FileEntry {
                            name: "lib.rs".to_string(),
                            path: current_dir.join("src/lib.rs"),
                            is_dir: false,
                            children: None,
                        },
                    ]),
                },
                FileEntry {
                    name: "Cargo.toml".to_string(),
                    path: current_dir.join("Cargo.toml"),
                    is_dir: false,
                    children: None,
                },
                FileEntry {
                    name: "README.md".to_string(),
                    path: current_dir.join("README.md"),
                    is_dir: false,
                    children: None,
                },
            ];
        }
    }
    
    /// Toggle sidebar visibility
    pub fn toggle_sidebar(&mut self) {
        self.sidebar_visible = !self.sidebar_visible;
    }
    
    /// Toggle panel visibility
    pub fn toggle_panel(&mut self) {
        self.panel_visible = !self.panel_visible;
    }

    /// Switch panel tab
    pub fn switch_panel_tab(&mut self, tab: PanelTab) {
        self.selected_panel_tab = tab;
    }

    /// Switch activity bar view
    pub fn switch_activity_view(&mut self, view: crate::tui::vscode::activity_bar::ActivityView) {
        self.active_view = Some(view);
    }

    /// Update the status bar with language information for the current file
    fn update_language_status(&mut self, path: &PathBuf) {
        // Detect language from file extension
        let language = self.syntax_highlighter.language_for_file(path);
        
        // Find or create language status item
        let language_item_idx = self.status_right_items.iter().position(|item| item.key == "language");
        if let Some(idx) = language_item_idx {
            self.status_right_items[idx].text = language;
        } else {
            self.status_right_items.push(StatusItem::new_key("language", language.as_str()));
        }
    }

    /// Open a file in the editor
    pub fn open_file_in_editor(&mut self, path: PathBuf) -> AppResult<()> {
        // Check if file already open
        let already_open = self.open_files.iter().position(|f| f.path == path);
        
        if let Some(idx) = already_open {
            // File already open, just switch to it
            self.active_tab_index = idx;
            return Ok(());
        }
        
        // Read file content
        match fs::read_to_string(&path) {
            Ok(content) => {
                // Create new file entry
                let file_name = path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("Untitled")
                    .to_string();
                    
                let file_entry = FileEntry {
                    name: file_name,
                    path: path.clone(),
                    is_dir: false,
                    children: None,
                };
                
                // Add to open files
                self.open_files.push(file_entry);
                
                // Set as active tab
                let new_index = self.open_files.len() - 1;
                self.active_tab_index = new_index;
                
                // Set file content
                self.file_contents = content.lines().map(String::from).collect();
                self.current_file = Some(path.clone());

                // Update language information in the status bar
                self.update_language_status(&path);
                
                
                self.modified = false;
                // Update Git information
                self.git_manager.detect_repo(&path);
                if self.git_manager.has_repo() {
                    // Update branch information if within a repo
                    self.git_branch = self.git_manager.branch().map(|b| b.to_string());
                }
                
                // Reset cursor and scroll
                self.code_view_state.selected_line = Some(0);
                self.code_view_state.cursor_position = 0;
                self.editor_scroll_offset = 0;
                
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Failed to open file: {}", e))
        }
    }

    /// Close a file tab
    pub fn close_file_tab(&mut self, index: usize) -> AppResult<()> {
        if index >= self.open_files.len() {
            return Err(anyhow::anyhow!("Invalid tab index"));
        }
        
        // Check if modified
        if self.active_tab_index == index && self.modified {
            // Ask for confirmation
            // In a real app, this would show a dialog
            return Err(anyhow::anyhow!("File has unsaved changes"));
        }
        
        // Remove the file
        self.open_files.remove(index);
        
        // Adjust active index
        if self.active_tab_index >= self.open_files.len() {
            self.active_tab_index = self.open_files.len().saturating_sub(1);
        }
        
        // If no files open, clear current file
        if self.open_files.is_empty() {
            self.current_file = None;
            self.file_contents.clear();
        }
        
        Ok(())
    }
    
    /// Toggle command palette visibility
    pub fn toggle_command_palette(&mut self) {
        self.command_palette_visible = !self.command_palette_visible;
        if self.command_palette_visible {
            self.command_palette_input = String::new();
            self.command_palette_selection = Some(0);
            self.mode = AppMode::Command;
        } else {
            self.mode = AppMode::Normal;
        }
    }

    /// Execute a command from the command palette
    pub fn execute_command_by_id(&mut self, command_id: &str) -> AppResult<()> {
        match command_id {
            "file.new" => {
                // Create new file
                self.file_contents = Vec::new();
                self.current_file = None;
                self.modified = false;
                
                // Add to open files
                let file_entry = FileEntry {
                    name: "Untitled".to_string(),
                    path: PathBuf::from("Untitled"),
                    is_dir: false,
                    children: None,
                };
                
                self.open_files.push(file_entry);
                self.active_tab_index = self.open_files.len() - 1;
                
                Ok(())
            },
            "file.open" => {
                // In a real app, this would open a file dialog
                // For now, just use a hard-coded example
                let example_path = PathBuf::from("examples/example.rs");
                self.open_file_in_editor(example_path)
            },
            "file.save" => self.save_file(),
            "view.toggleSidebar" => {
                self.toggle_sidebar();
                Ok(())
            },
            "view.togglePanel" => {
                self.toggle_panel();
                Ok(())
            },
            "debug.start" => self.run_vm(),
            "debug.pause" => self.pause_vm(),
            "debug.stop" => self.stop_vm(),
            "debug.stepOver" => self.step_vm(),
            "view.toggleUiStyle" => {
                self.toggle_ui_style();
                Ok(())
            },
            _ => {
                // Unknown command
                Err(anyhow::anyhow!("Unknown command: {}", command_id))
            }
        }
    }

    /// Add a problem to the problems panel
    pub fn add_problem(&mut self, file: String, line: usize, column: usize, message: String, severity: String) {
        self.problems.push(Problem {
            file,
            line,
            column,
            message,
            severity,
        });
    }
    
    /// Initialize the application with a file
    pub fn init(&mut self, path: Option<PathBuf>) -> AppResult<()> {
        // Add initial console message
        self.add_to_console("lessVM TUI IDE started".to_string());
        
        // Select the first console message
        self.console_list_state.select(Some(0));
        
        // Load file if provided
        if let Some(path) = path {
            self.load_file(&path)?;
        }
        
        Ok(())
    }
    
    /// Run on every tick
    pub fn tick(&mut self) -> AppResult<()> {
        // Only run VM if it's in running state
        if self.vm_status == VMStatus::Running {
            // Execute a single step
            match self.vm.step() {
                Ok(continue_execution) => {
                    // Check if at breakpoint
                    let pc = self.vm.get_program_counter();
                    if self.breakpoints.contains_key(&pc) {
                        self.vm_status = VMStatus::Paused;
                        self.add_to_console(format!("Breakpoint hit at {:#x}", pc));
                    }
                    
                    // Check for watchpoints
                    // Collect watchpoints that were triggered
                    let mut triggered_watchpoints = Vec::new();
                    if let Some(old_mem) = &self.memory_snapshot {
                        let new_mem = self.vm.get_memory();
                        for addr in self.watchpoints.keys() {
                            if *addr < old_mem.len() && *addr < new_mem.len() {
                                if old_mem[*addr] != new_mem[*addr] {
                                    triggered_watchpoints.push((*addr, old_mem[*addr], new_mem[*addr]));
                                }
                            }
                        }
                        // Process triggered watchpoints
                        if !triggered_watchpoints.is_empty() {
                            self.vm_status = VMStatus::Paused;
                            for (addr, old_val, new_val) in triggered_watchpoints {
                                self.add_to_console(format!(
                                    "Watchpoint hit at {:#x}: {} -> {}",
                                    addr, old_val, new_val
                                    ));
                            }
                        }
                    }
                    
                    // Update memory snapshot
                    self.take_memory_snapshot();
                    
                    // Check if program has terminated
                    if !continue_execution || self.vm.is_terminated() {
                        self.vm_status = VMStatus::Stopped;
                        self.add_to_console("Program terminated".to_string());
                    }
                }
                Err(e) => {
                    self.vm_status = VMStatus::Stopped;
                    self.last_error = Some(format!("VM error: {}", e));
                    self.add_to_console(format!("ERROR: {}", e));
                }
            }
        }
        
        Ok(())
    }
    
    /// Load a file
    pub fn load_file(&mut self, path: &Path) -> AppResult<()> {
        // Read file contents
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;
        
        // Split into lines
        self.file_contents = content.lines().map(|s| s.to_string()).collect();
        
        // Set current file
        self.current_file = Some(path.to_path_buf());
        self.modified = false;

        // Update language information in the status bar
        self.update_language_status(&path.to_path_buf());
        
        // Update Git information
        self.git_manager.detect_repo(path);
        if self.git_manager.has_repo() {
            self.git_branch = self.git_manager.branch().map(|b| b.to_string());
        }
        
        // Reset cursor position
        self.code_view_state.selected_line = Some(0);
        self.code_view_state.cursor_position = 0;
        self.code_view_state.offset = 0;
        
        // Add to console
        self.add_to_console(format!("Loaded file: {}", path.display()));
        
        Ok(())
    }
    
    /// Save current file
    pub fn save_file(&mut self) -> AppResult<()> {
        if let Some(path) = &self.current_file {
            // Join lines and write to file
            let content = self.file_contents.join("\n");
            fs::write(path, content)
                .with_context(|| format!("Failed to write file: {}", path.display()))?;
            
            // Update state
            self.modified = false;
            
            // Add to console
            self.add_to_console(format!("Saved file: {}", path.display()));
            
            Ok(())
        } else {
            // No file loaded
            self.dialog = Some(DialogType::Input(
                "Enter file path to save:".to_string(),
                String::new(),
                Box::new(|app, path| {
                    if !path.is_empty() {
                        app.current_file = Some(PathBuf::from(path));
                        match app.save_file() {
                            Ok(()) => {}
                            Err(e) => {
                                app.last_error = Some(e.to_string());
                                app.add_to_console(format!("ERROR: {}", e));
                            }
                        }
                    }
                }),
            ));
            
            Ok(())
        }
    }
    
    /// Add a message to the console
    pub fn add_to_console(&mut self, message: String) {
        self.console_history.push(message);
        
        // Limit console history to 1000 lines
        if self.console_history.len() > 1000 {
            self.console_history.remove(0);
        }
        
        // Update selection to follow latest message
        let new_index = self.console_history.len() - 1;
        self.console_list_state.select(Some(new_index));
    }
    
    /// Execute a command
    pub fn execute_command(&mut self, command: &str) -> AppResult<()> {
        // Add command to history
        self.command_history.push(command.to_string());
        self.command_history_index = self.command_history.len();
        
        // Add to console
        self.add_to_console(format!("> {}", command));
        
        // Parse command
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }
        
        match parts[0] {
            // Run VM
            "run" | "r" => self.run_vm(),
            
            // Step VM
            "step" | "s" => self.step_vm(),
            
            // Pause VM
            "pause" | "p" => self.pause_vm(),
            
            // Stop VM
            "stop" => self.stop_vm(),
            
            // Reset VM
            "reset" => self.reset_vm(),
            
            // Toggle breakpoint
            "break" | "b" => {
                if parts.len() > 1 {
                    if let Ok(line) = parts[1].parse::<usize>() {
                        self.toggle_breakpoint(line - 1);
                        self.add_to_console(format!("Toggled breakpoint at line {}", line));
                    } else {
                        self.add_to_console(format!("Invalid line number: {}", parts[1]));
                    }
                } else {
                    // Toggle at current line
                    if let Some(line) = self.code_view_state.selected_line {
                        self.toggle_breakpoint(line);
                        self.add_to_console(format!("Toggled breakpoint at line {}", line + 1));
                    }
                }
                Ok(())
            },
            
            // Clear breakpoint
            "clear" => {
                if parts.len() > 1 {
                    if parts[1] == "all" {
                        // Clear all breakpoints
                        self.breakpoints.clear();
                        self.add_to_console("Cleared all breakpoints".to_string());
                    } else if let Ok(line) = parts[1].parse::<usize>() {
                        // Clear specific breakpoint
                        self.breakpoints.remove(&(line - 1));
                        self.add_to_console(format!("Cleared breakpoint at line {}", line));
                    } else {
                        self.add_to_console(format!("Invalid line number: {}", parts[1]));
                    }
                } else {
                    // Clear at current line
                    if let Some(line) = self.code_view_state.selected_line {
                        self.breakpoints.remove(&line);
                        self.add_to_console(format!("Cleared breakpoint at line {}", line + 1));
                    }
                }
                Ok(())
            },
            
            // Add memory watchpoint
            "watch" | "w" => {
                if parts.len() > 1 {
                    if let Ok(addr) = parse_address(parts[1]) {
                        self.watchpoints.insert(addr, true);
                        self.take_memory_snapshot();
                        self.add_to_console(format!("Added watchpoint at {:#x}", addr));
                    } else {
                        self.add_to_console(format!("Invalid address: {}", parts[1]));
                    }
                } else {
                    // Add at current address
                    if let Some(addr) = self.memory_view_state.selected_address {
                        self.watchpoints.insert(addr, true);
                        self.take_memory_snapshot();
                        self.add_to_console(format!("Added watchpoint at {:#x}", addr));
                    }
                }
                Ok(())
            },
            
            // Load file
            "load" => {
                if parts.len() > 1 {
                    let path = PathBuf::from(parts[1]);
                    match self.load_file(&path) {
                        Ok(()) => {}
                        Err(e) => {
                            self.last_error = Some(e.to_string());
                            self.add_to_console(format!("ERROR: {}", e));
                        }
                    }
                } else {
                    self.add_to_console("Missing file path".to_string());
                }
                Ok(())
            },
            
            // Save file
            "save" => self.save_file(),
            
            // Load into VM and disassemble
            "assemble" | "asm" => {
                self.assemble_current_file()
            },
            
            // Unknown command
            _ => {
                self.add_to_console(format!("Unknown command: {}", parts[0]));
                Ok(())
            },
        }
    }
    
    /// Toggle breakpoint at a line
    pub fn toggle_breakpoint(&mut self, line: usize) {
        if self.breakpoints.contains_key(&line) {
            self.breakpoints.remove(&line);
        } else {
            self.breakpoints.insert(line, true);
        }
    }
    
    /// Take a snapshot of VM memory
    pub fn take_memory_snapshot(&mut self) {
        self.memory_snapshot = Some(self.vm.get_memory().to_vec());
    }
    
    /// Run the VM
    pub fn run_vm(&mut self) -> AppResult<()> {
        if self.vm_status == VMStatus::Stopped {
            // If stopped, assemble and load first
            self.assemble_current_file()?;
        }
        
        self.vm_status = VMStatus::Running;
        self.add_to_console("VM running".to_string());
        Ok(())
    }
    
    /// Step the VM
    pub fn step_vm(&mut self) -> AppResult<()> {
        if self.vm_status == VMStatus::Stopped {
            // If stopped, assemble and load first
            self.assemble_current_file()?;
            self.vm_status = VMStatus::Paused;
        }
        
        if self.vm_status == VMStatus::Paused {
            // Execute a single step
            match self.vm.step() {
                Ok(continue_execution) => {
                    self.add_to_console(format!("Stepped to PC: {:#x}", self.vm.get_program_counter()));
                    
                    // Update memory snapshot
                    self.take_memory_snapshot();
                    
                    // Check if program has terminated
                    if !continue_execution || self.vm.is_terminated() {
                        self.vm_status = VMStatus::Stopped;
                        self.add_to_console("Program terminated".to_string());
                    }
                }
                Err(e) => {
                    self.vm_status = VMStatus::Stopped;
                    self.last_error = Some(format!("VM error: {}", e));
                    self.add_to_console(format!("ERROR: {}", e));
                }
            }
        }
        
        Ok(())
    }
    
    /// Pause the VM
    pub fn pause_vm(&mut self) -> AppResult<()> {
        if self.vm_status == VMStatus::Running {
            self.vm_status = VMStatus::Paused;
            self.add_to_console("VM paused".to_string());
        }
        Ok(())
    }
    
    /// Stop the VM
    pub fn stop_vm(&mut self) -> AppResult<()> {
        if self.vm_status != VMStatus::Stopped {
            self.vm_status = VMStatus::Stopped;
            self.add_to_console("VM stopped".to_string());
        }
        Ok(())
    }
    
    /// Reset the VM
    pub fn reset_vm(&mut self) -> AppResult<()> {
        self.vm.reset();
        self.vm_status = VMStatus::Stopped;
        self.disassembly.clear();
        self.add_to_console("VM reset".to_string());
        Ok(())
    }
    
    /// Assemble current file and load into VM
    pub fn assemble_current_file(&mut self) -> AppResult<()> {
        // Join lines and assemble
        let source = self.file_contents.join("\n");
        
        // For now, we'll just add placeholders for disassembly
        // In a real implementation, this would actually assemble and load the VM
        
        // Reset VM
        self.vm.reset();
        
        // Generate placeholder disassembly
        self.disassembly = source.lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty() && !line.trim().starts_with(';'))
            .map(|(i, line)| format!("{:#04x}: {}", i, line.trim()))
            .collect();
        
        // Set VM status
        self.vm_status = VMStatus::Paused;
        
        // Take memory snapshot
        self.take_memory_snapshot();
        
        self.add_to_console("Assembled and loaded program".to_string());
        Ok(())
    }
}

/// Parse an address from a string
/// 
/// Supports decimal (123), hexadecimal (0x7B), and binary (0b1111011) formats
fn parse_address(addr_str: &str) -> Result<usize, std::num::ParseIntError> {
    if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
        // Hexadecimal
        usize::from_str_radix(&addr_str[2..], 16)
    } else if addr_str.starts_with("0b") || addr_str.starts_with("0B") {
        // Binary
        usize::from_str_radix(&addr_str[2..], 2)
    } else {
        // Decimal
        addr_str.parse::<usize>()
    }
}