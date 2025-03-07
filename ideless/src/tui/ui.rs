//! TUI rendering
//!
//! This module handles rendering of the TUI.
//! Supports both classic IDE mode and VS Code-like mode.

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Tabs, Wrap,
    },
    Frame,
};

use super::app::{App, AppMode, DialogType, Tab, VMStatus};
use super::widgets::{CodeViewState, DisassemblyView, MemoryViewState, RegisterView, StackView};
use super::vscode_ui;

/// Render the UI
pub fn render(app: &mut App, f: &mut Frame) {
    // Determine which UI style to use - note: we need to cast to mutable for vscode_ui
    if app.use_vscode_ui {
        // Render VS Code-like UI
        vscode_ui::render(app, f);
    } else {
        // Render classic UI
        render_classic_ui(app, f);
    }
    
    // Always render dialog if present 
    if app.dialog.is_some() {
        render_dialog::<tui::backend::CrosstermBackend<std::io::Stdout>>(app, f);
    }
}

/// Render the classic IDE UI
fn render_classic_ui(app: &mut App, f: &mut Frame) {
    // Create layout for classic UI
    let chunks = Layout::default().direction(Direction::Vertical).constraints([
        Constraint::Length(3),  // Tab bar
        Constraint::Min(0),     // Main content
        Constraint::Length(1),  // Command input
        Constraint::Length(1),  // Status line
    ]).split(f.size());
    
    // Render tab bar
    render_tabs(app, f, chunks[0]);
    
    // Render main content based on selected tab
    match app.current_tab {
        Tab::Editor => render_editor(app, f, chunks[1]),
        Tab::Disassembler => render_disassembler(app, f, chunks[1]),
        Tab::VMState => render_vm_state(app, f, chunks[1]),
        Tab::Memory => render_memory::<tui::backend::CrosstermBackend<std::io::Stdout>>(app, f, chunks[1]),
        Tab::Console => render_console::<tui::backend::CrosstermBackend<std::io::Stdout>>(app, f, chunks[1]),
    }
    
    // Render command input
    render_command_input::<tui::backend::CrosstermBackend<std::io::Stdout>>(app, f, chunks[2]);
    
    // Render status line
    render_status_line::<tui::backend::CrosstermBackend<std::io::Stdout>>(app, f, chunks[3]);
}

/// Render the tabs
fn render_tabs(app: &mut App, f: &mut Frame, rect: Rect) {
    let titles = vec![
        "1:Editor",
        "2:Disassembler",
        "3:VM State",
        "4:Memory",
        "5:Console",
    ];
    
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("ideless TUI"))
        .select(app.current_tab as usize)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );
    
    f.render_widget(tabs, rect);
}

/// Render the editor
fn render_editor(app: &mut App, f: &mut Frame, rect: Rect) {
    // Create editor layout with line numbers and content
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(6),  // Line numbers
            Constraint::Min(0),     // Content
        ])
        .split(rect);
    
    // Show file name in title
    let title = if let Some(path) = &app.current_file {
        format!("File: {} {}", path.display(), if app.modified { "*" } else { "" })
    } else {
        "No file loaded".to_string()
    };
    
    // Render line numbers
    render_line_numbers(app, f, chunks[0]);
    
    // Render content
    let lines: Vec<Line> = app.file_contents.iter().enumerate()
        .skip(app.code_view_state.offset)
        .take((rect.height - 2) as usize)
        .map(|(i, line)| {
            // Determine style based on selection and breakpoints
            let style = if app.code_view_state.selected_line == Some(i) {
                Style::default().bg(Color::DarkGray)
            } else if app.breakpoints.contains_key(&i) {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            };
            
            Line::from(Span::styled(line, style))
        })
        .collect();
    
    let content = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false });
    
    f.render_widget(content, chunks[1]);
    
    // Render cursor if in edit mode
    if app.mode == AppMode::Edit {
        if let Some(line_idx) = app.code_view_state.selected_line {
            if line_idx >= app.code_view_state.offset && 
               (line_idx - app.code_view_state.offset) < chunks[1].height as usize - 2 {
                // Calculate cursor position
                let line_pos = line_idx - app.code_view_state.offset;
                let cursor_x = app.code_view_state.cursor_position;
                
                // Position cursor
                f.set_cursor(
                    chunks[1].x + 1 + cursor_x as u16,
                    chunks[1].y + 1 + line_pos as u16
                );
            }
        }
    }
}

/// Render line numbers
fn render_line_numbers(app: &mut App, f: &mut Frame, rect: Rect) {
    let lines: Vec<Line> = (app.code_view_state.offset..(app.code_view_state.offset + rect.height as usize - 2))
        .map(|i| {
            // Determine style based on breakpoints
            let style = if app.breakpoints.contains_key(&i) {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            
            let num = format!("{:4} ", i + 1);
            Line::from(Span::styled(num, style))
        })
        .collect();
    
    let line_numbers = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Lines"))
        .wrap(Wrap { trim: false });
    
    f.render_widget(line_numbers, rect);
}

/// Render the disassembler
fn render_disassembler(app: &mut App, f: &mut Frame, rect: Rect) {
    // Prepare disassembly content
    let disasm_lines: Vec<ListItem> = app.disassembly.iter().enumerate()
        .map(|(i, line)| {
            // Highlight if this is the current instruction
            let style = if app.vm_status != VMStatus::Stopped && 
                        DisassemblyView::is_current_instruction(i, app.vm.get_program_counter()) {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            let formatted = DisassemblyView::format_line(i, line);
            ListItem::new(formatted).style(style)
        })
        .collect();
    
    let disasm_list = List::new(disasm_lines)
        .block(Block::default().borders(Borders::ALL).title("Disassembly"))
        .highlight_style(Style::default().bg(Color::DarkGray));
    
    f.render_widget(disasm_list, rect);
}

/// Render the VM state tab
fn render_vm_state(app: &mut App, f: &mut Frame, rect: Rect) {
    // Create layout for registers and stack
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),  // Registers
            Constraint::Percentage(50),  // Stack
        ])
        .split(rect);
    
    // Render registers
    let registers = app.vm.get_registers();
    let reg_items: Vec<ListItem> = registers.iter().enumerate()
        .map(|(i, &value)| {
            let text = RegisterView::format_register(&format!("r{}", i), &value.to_string());
            ListItem::new(text)
        })
        .collect();
    
    let reg_list = List::new(reg_items)
        .block(Block::default().borders(Borders::ALL).title("Registers"))
        .highlight_style(Style::default().bg(Color::DarkGray));
    
    f.render_widget(reg_list, chunks[0]);
    
    // Render stack
    let stack = app.vm.get_stack();
    let stack_items: Vec<ListItem> = stack.iter().enumerate()
        .map(|(i, &value)| {
            let text = StackView::format_stack_item(i, &value.to_string());
            ListItem::new(text)
        })
        .collect();
    
    let stack_list = List::new(stack_items)
        .block(Block::default().borders(Borders::ALL).title("Stack"))
        .highlight_style(Style::default().bg(Color::DarkGray));
    
    f.render_widget(stack_list, chunks[1]);
}

/// Render the memory view
fn render_memory<B: Backend>(app: &mut App, f: &mut Frame, rect: Rect) {
    // Get memory data
    let memory = app.vm.get_memory();
    let memory_snapshot = app.memory_snapshot.as_ref();
    let selected_address = app.memory_view_state.selected_address;
    
    // Create memory view text lines
    let mut lines = Vec::new();
    let bytes_per_row = app.memory_view_state.bytes_per_line;
    let visible_rows = (rect.height - 2) as usize;
    
    for row in 0..visible_rows {
        let base_addr = app.memory_view_state.address + (row * bytes_per_row);
        if base_addr >= memory.len() {
            break;
        }
        
        // Address column
        let mut line = vec![
            Span::styled(
                format!("{:#08x} | ", base_addr),
                Style::default().fg(Color::Blue)
            )
        ];
        
        // Hex values
        for col in 0..bytes_per_row {
            let addr = base_addr + col;
            if addr >= memory.len() {
                line.push(Span::raw("   "));
                continue;
            }
            
            let byte = memory[addr];
            let is_selected = selected_address == Some(addr);
            let has_changed = app.memory_view_state.has_address_changed(addr);
            
            let style = match (is_selected, has_changed) {
                (true, true) => Style::default().fg(Color::Red).bg(Color::DarkGray),
                (true, false) => Style::default().bg(Color::DarkGray),
                (false, true) => Style::default().fg(Color::Red),
                (false, false) => Style::default(),
            };
            
            line.push(Span::styled(
                format!("{:02x} ", byte),
                style
            ));
        }
        
        // Separator
        line.push(Span::raw(" | "));
        
        // ASCII representation
        for col in 0..bytes_per_row {
            let addr = base_addr + col;
            if addr >= memory.len() {
                line.push(Span::raw(" "));
                continue;
            }
            
            let byte = memory[addr];
            let is_selected = selected_address == Some(addr);
            let has_changed = app.memory_view_state.has_address_changed(addr);
            
            let style = match (is_selected, has_changed) {
                (true, true) => Style::default().fg(Color::Red).bg(Color::DarkGray),
                (true, false) => Style::default().bg(Color::DarkGray),
                (false, true) => Style::default().fg(Color::Red),
                (false, false) => Style::default(),
            };
            
            line.push(Span::styled(
                app.memory_view_state.format_byte_as_ascii(byte).to_string(),
                style
            ));
        }
        
        lines.push(Line::from(line));
    }
    
    let memory_paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Memory"));
    
    f.render_widget(memory_paragraph, rect);
}

/// Render the console
fn render_console<B: Backend>(app: &mut App, f: &mut Frame, rect: Rect) {
    let console_items: Vec<ListItem> = app.console_history.iter()
        .map(|line| {
            // Stylize based on content
            let style = if line.starts_with("ERROR") {
                Style::default().fg(Color::Red)
            } else if line.starts_with(">") {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            ListItem::new(line.as_str()).style(style)
        })
        .collect();
    
    let console_list = List::new(console_items)
        .block(Block::default().borders(Borders::ALL).title("Console"))
        .highlight_style(Style::default().bg(Color::DarkGray));
    
    f.render_stateful_widget(console_list, rect, &mut app.console_list_state.clone());
}

/// Render the command input
fn render_command_input<B: Backend>(app: &mut App, f: &mut Frame, rect: Rect) {
    // Only show command input when in command mode
    let text = if app.mode == AppMode::Command {
        format!(": {}", app.command_input)
    } else {
        "".to_string()
    };
    
    let command_input = Paragraph::new(text)
        .style(Style::default());
    
    f.render_widget(command_input, rect);
    
    // Show cursor in command input if in command mode
    if app.mode == AppMode::Command {
        f.set_cursor(
            rect.x + 2 + app.command_input.len() as u16,
            rect.y
        );
    }
}

/// Render the status line
fn render_status_line<B: Backend>(app: &mut App, f: &mut Frame, rect: Rect) {
    let mode_text = match app.mode {
        AppMode::Normal => "NORMAL",
        AppMode::Edit => "EDIT",
        AppMode::Command => "COMMAND",
        AppMode::Dialog => "DIALOG",
        AppMode::Help => "HELP",
    };
    
    let vm_status = match app.vm_status {
        VMStatus::Stopped => "Stopped",
        VMStatus::Running => "Running",
        VMStatus::Paused => "Paused",
    };
    
    let position = if let Some(line) = app.code_view_state.selected_line {
        format!("{}:{}", line + 1, app.code_view_state.cursor_position + 1)
    } else {
        "0:0".to_string()
    };
    
    let file_info = if let Some(path) = &app.current_file {
        let filename = path.file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled".to_string());
        
        format!("{}{}", filename, if app.modified { "*" } else { "" })
    } else {
        "No file".to_string()
    };
    
    // Construct status line
    let status = format!(" {} | {} | VM: {} | {} ", 
        mode_text, position, vm_status, file_info);
    
    // Show last error if any
    let error_part = if let Some(error) = &app.last_error {
        format!(" | ERROR: {} ", error)
    } else {
        "".to_string()
    };
    
    // Construct full status text
    let full_status = format!("{}{}", status, error_part);
    
    let mut text = Text::from(full_status);
    text.patch_style(Style::default().bg(Color::Blue).fg(Color::White));
    
    let status_paragraph = Paragraph::new(text);
    f.render_widget(status_paragraph, rect);
}

/// Render a dialog
fn render_dialog<B: Backend>(app: &mut App, f: &mut Frame) {
    if let Some(dialog) = &app.dialog {
        // Calculate dialog size and position
        let width = 60u16;
        let height = 10u16;
        let x = (f.size().width - width) / 2;
        let y = (f.size().height - height) / 2;
        
        let dialog_rect = Rect::new(x, y, width, height);
        
        match dialog {
            DialogType::Confirm(message, _) => {
                let text = format!("{}\n\nPress Y to confirm, N to cancel", message);
                let paragraph = Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL).title("Confirm"))
                    .wrap(Wrap { trim: true });
                
                f.render_widget(paragraph, dialog_rect);
            },
            DialogType::Input(title, value, _) => {
                let text = format!("{}\n\n{}", title, value);
                let paragraph = Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL).title("Input"))
                    .wrap(Wrap { trim: true });
                
                f.render_widget(paragraph, dialog_rect);
                
                // Position cursor at end of input
                f.set_cursor(
                    dialog_rect.x + value.len() as u16 + 1,
                    dialog_rect.y + 3
                );
            },
            DialogType::Message(message) => {
                let text = format!("{}\n\nPress ENTER or ESC to close", message);
                let paragraph = Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL).title("Message"))
                    .wrap(Wrap { trim: true });
                
                f.render_widget(paragraph, dialog_rect);
            },
        }
    }
}

/// Render help screen
fn render_help<B: Backend>(app: &mut App, f: &mut Frame, rect: Rect) {
    // Help text
    let help_text = "
ideless TUI Help

Navigation:
  Tab / 1-5      - Switch between tabs
  Arrow keys / hjkl - Navigate
  PgUp/PgDown    - Page up/down
  Home/End       - Jump to start/end

Editor:
  i              - Enter edit mode
  Esc            - Exit edit mode

Commands:
  :              - Enter command mode
  run / r        - Run VM
  step / s       - Step VM
  pause / p      - Pause VM
  stop           - Stop VM
  reset          - Reset VM
  break / b <n>  - Toggle breakpoint at line n
  clear <n>      - Clear breakpoint at line n
  clear all      - Clear all breakpoints
  watch / w <addr> - Watch memory at address
  load <file>    - Load file
  save           - Save current file

Function Keys:
  F1             - Help
  F5             - Run
  F6             - Step
  F7             - Pause
  F8             - Stop
  F9             - Reset

Other:
  Ctrl+S         - Save file
  q              - Quit
  ?              - Show this help

Press ESC or ? to close help
";

    // Calculate dialog size and position
    let width = rect.width.min(80);
    let height = rect.height.min(30);
    let x = (rect.width - width) / 2;
    let y = (rect.height - height) / 2;
    
    let help_rect = Rect::new(x, y, width, height);
    
    let paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: false });
    
    f.render_widget(paragraph, help_rect);
}