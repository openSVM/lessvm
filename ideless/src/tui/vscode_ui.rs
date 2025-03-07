//! VS Code-like UI renderer
//!
//! This module renders a VS Code-like interface in the terminal using Ratatui.
//! 

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    layout::Margin,
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap},
    Frame,
}; 

use crate::tui::app::{App, AppMode, PanelTab, Tab, VMStatus};
use crate::tui::vscode::activity_bar::ActivityView;
use crate::tui::vscode::{self, activity_bar, theme::VscodeTheme};



/// Render VS Code-like UI with a generic backend
pub fn render(app: &mut App, f: &mut Frame) {
    // Define main layout areas - activity bar, sidebar, main content, status bar
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),      // Main area
            Constraint::Length(1),   // Status bar
        ])
        .split(f.size());
    
    // Main area with activity bar, sidebar, and editor area
    let main_area = main_layout[0];
    let status_bar_area = main_layout[1];
    
    // Determine sidebar width
    let sidebar_width = if app.sidebar_visible { 30 } else { 0 };
    
    // Determine panel height
    let panel_height = if app.panel_visible { 10 } else { 0 };
    
    // Split main area horizontally: activity bar, sidebar, editor/panel area
    let horizontal_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(3),                     // Activity bar
            Constraint::Length(sidebar_width),         // Sidebar (if visible)
            Constraint::Min(10),                       // Editor area
        ])
        .split(main_area);
    
    // Activity bar
    let activity_bar_area = horizontal_split[0];
    let sidebar_area = if app.sidebar_visible { horizontal_split[1] } else { Rect::default() };
    let content_area = horizontal_split[2];
    
    // Split content area vertically for editor and panel
    let editor_panel_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),                        // Editor area
            Constraint::Length(panel_height),          // Panel area (if visible)
        ])
        .split(content_area);
    
    let editor_area = editor_panel_split[0];
    let panel_area = if app.panel_visible { editor_panel_split[1] } else { Rect::default() };
    
    // Render components
    vscode::activity_bar::render_activity_bar(app, f, activity_bar_area);
    
    if app.sidebar_visible {
        render_sidebar(app, f, sidebar_area);
    }
    
    render_editor_area(app, f, editor_area);
    
    if app.panel_visible {
        render_panel_area(app, f, panel_area);
    }
    
    vscode::status_bar::render(app, f, status_bar_area);
    
    // Command palette overlay (if visible)
    if app.command_palette_visible {
        render_command_palette(app, f);
    }
    
    // Render dialog if present
    if app.dialog.is_some() {
        render_dialog(app, f);
    }
}

/// Render sidebar with content based on active view
fn render_sidebar(app: &App, f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(VscodeTheme::BORDER))
        .style(Style::default().bg(VscodeTheme::SIDEBAR_BACKGROUND));
    
    f.render_widget(block, area);
    
    // Layout for sidebar title and content
    let sidebar_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),      // Title
            Constraint::Min(3),         // Content
        ])
        .split(area.inner(&Margin { horizontal: 1, vertical: 0 })); // Remove right border from area
    
    // Render title based on active view
    let title = match app.active_view {
        Some(ActivityView::Explorer) => "EXPLORER",
        Some(ActivityView::Search) => "SEARCH",
        Some(ActivityView::SourceControl) => "SOURCE CONTROL",
        Some(ActivityView::Debug) => "DEBUG",
        Some(ActivityView::Extensions) => "EXTENSIONS",
        None => "EXPLORER",
    };
    
    let title_paragraph = Paragraph::new(title)
        .style(Style::default()
            .fg(VscodeTheme::TEXT)
            .bg(VscodeTheme::SIDEBAR_BACKGROUND))
        .block(Block::default().style(Style::default().bg(VscodeTheme::SIDEBAR_BACKGROUND)));
    
    f.render_widget(title_paragraph, sidebar_layout[0]);
    
    // Render content based on active view
    match app.active_view {
        Some(ActivityView::Explorer) => render_explorer_view(app, f, sidebar_layout[1]),
        Some(ActivityView::Search) => render_search_view(app, f, sidebar_layout[1]),
        Some(ActivityView::SourceControl) => render_source_control_view(app, f, sidebar_layout[1]),
        Some(ActivityView::Debug) => render_debug_view(app, f, sidebar_layout[1]),
        Some(ActivityView::Extensions) => render_extensions_view(app, f, sidebar_layout[1]),
        None => render_explorer_view(app, f, sidebar_layout[1]),
    }
}

/// Render explorer view in sidebar
fn render_explorer_view(app: &App, f: &mut Frame, area: Rect) {
    // Workspace title
    let default_workspace = "LESSVM PROJECT".to_string();
    let workspace_name = app.current_workspace_name.as_ref()
        .unwrap_or(&default_workspace);
    
    let workspace_title = format!("{} ▾", workspace_name);
    
    let title_paragraph = Paragraph::new(workspace_title)
        .style(Style::default()
            .fg(VscodeTheme::TEXT)
            .bg(VscodeTheme::SIDEBAR_BACKGROUND));
    
    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),      // Workspace title
            Constraint::Min(3),         // File tree
        ])
        .split(area);
    
    f.render_widget(title_paragraph, inner_layout[0]);
    
    // File explorer tree
    let mut file_items = Vec::new();
    
    // Recursively build file tree items
    for (i, entry) in app.file_explorer.iter().enumerate() {
        let selected = app.selected_file_idx.map_or(false, |idx| idx == i);
        
        let style = if selected {
            Style::default()
                .fg(VscodeTheme::SELECTED_TEXT)
                .bg(VscodeTheme::SELECTED_BACKGROUND)
        } else {
            Style::default().fg(VscodeTheme::TEXT)
        };
        
        let icon = if entry.is_dir { "📁 " } else { "📄 " };
        let entry_text = format!("{}{}", icon, entry.name);
        
        file_items.push(ListItem::new(entry_text).style(style));
        
        // Add children if this is a directory
        if entry.is_dir && entry.children.is_some() {
            for child in entry.children.as_ref().unwrap() {
                let child_icon = if child.is_dir { "📁 " } else { "📄 " };
                let child_text = format!("  {}{}", child_icon, child.name);
                file_items.push(ListItem::new(child_text).style(Style::default().fg(VscodeTheme::TEXT)));
            }
        }
    }
    
    let file_list = List::new(file_items)
        .block(Block::default().style(Style::default().bg(VscodeTheme::SIDEBAR_BACKGROUND)))
        .highlight_style(
            Style::default()
                .bg(VscodeTheme::SELECTED_BACKGROUND)
                .add_modifier(Modifier::BOLD),
        );
    
    f.render_widget(file_list, inner_layout[1]);
}

/// Render search view in sidebar
fn render_search_view(app: &App, f: &mut Frame, area: Rect) {
    let search_text = Paragraph::new("Search (press Ctrl+F to search)")
        .style(Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::SIDEBAR_BACKGROUND));
    
    f.render_widget(search_text, area);
}

/// Render source control view in sidebar
fn render_source_control_view(app: &App, f: &mut Frame, area: Rect) {
    let default_branch = "main".to_string();
    let git_branch = app.git_branch.as_ref().unwrap_or(&default_branch);
    let branch_text = format!("Branch: {}", git_branch);
    
    let source_control_text = Paragraph::new(vec![
        Line::from(branch_text),
        Line::from(""),
        Line::from("Changes:"),
        Line::from(" M ideless/src/tui/mod.rs"),
        Line::from(" M ideless/src/tui/app.rs"),
        Line::from(" A ideless/src/tui/vscode/mod.rs"),
        Line::from(" A ideless/src/tui/vscode_ui.rs"),
    ])
    .style(Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::SIDEBAR_BACKGROUND))
    .wrap(Wrap { trim: false });
    
    f.render_widget(source_control_text, area);
}

/// Render debug view in sidebar
fn render_debug_view(app: &App, f: &mut Frame, area: Rect) {
    let debug_status = match app.vm_status {
        VMStatus::Running => "Running",
        VMStatus::Paused => "Paused",
        VMStatus::Stopped => "Stopped",
    };
    
    let debug_text = Paragraph::new(vec![
        Line::from("VARIABLES"),
        Line::from(""),
        Line::from(format!("VM Status: {}", debug_status)),
        Line::from("PC: 0x00"),
        Line::from(""),
        Line::from("BREAKPOINTS"),
        Line::from(""),
    ])
    .style(Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::SIDEBAR_BACKGROUND))
    .wrap(Wrap { trim: false });
    
    f.render_widget(debug_text, area);
}

/// Render extensions view in sidebar
fn render_extensions_view(app: &App, f: &mut Frame, area: Rect) {
    let extensions_text = Paragraph::new(vec![
        Line::from("Installed Extensions:"),
        Line::from(""),
        Line::from("Rust Analyzer"),
        Line::from("LessVM Debugger"),
        Line::from("Git Integration"),
        Line::from("Syntax Highlighter"),
    ])
    .style(Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::SIDEBAR_BACKGROUND))
    .wrap(Wrap { trim: false });
    
    f.render_widget(extensions_text, area);
}

/// Render editor area with tabs and content
fn render_editor_area(app: &mut App, f: &mut Frame, area: Rect) {
    let editor_block = Block::default()
        .style(Style::default().bg(VscodeTheme::EDITOR_BACKGROUND));
    
    f.render_widget(editor_block, area);
    
    // Split area for tabs and content
    let editor_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),      // Tabs
            Constraint::Min(3),         // Content
        ])
        .split(area);
    
    let tabs_area = editor_layout[0];
    let content_area = editor_layout[1];
    
    // Render editor tabs
    render_editor_tabs(app, f, tabs_area);
    
    // Render file content
    if !app.file_contents.is_empty() {
        render_file_content(app, f, content_area);
    } else {
        let welcome_text = Paragraph::new(vec![
            Line::from(""),
            Line::from("Welcome to ideless - VS Code-like TUI for lessVM development"),
            Line::from(""),
            Line::from("Press Ctrl+O to open a file or Ctrl+N to create a new file"),
            Line::from("Press F1 or Ctrl+Shift+P for command palette"),
            Line::from(""),
            Line::from("Press Ctrl+B to toggle sidebar"),
            Line::from("Press Ctrl+J to toggle panel"),
        Line::from(""),
        ])
        .style(Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::EDITOR_BACKGROUND))
        .alignment(tui::layout::Alignment::Center)
        .wrap(Wrap { trim: false });
        
        f.render_widget(welcome_text, content_area);
    }
}

/// Render editor tabs
fn render_editor_tabs(app: &App, f: &mut Frame, area: Rect) {
    let tab_titles: Vec<Line> = if app.open_files.is_empty() {
        vec![Line::from("untitled")]
    } else {
        app.open_files
            .iter()
            .map(|file| {
                let mut title = file.name.clone();
                if &file.path == app.current_file.as_ref().unwrap_or(&std::path::PathBuf::new()) && app.modified {
                    title.push('*');
                }
                Line::from(title)
            })
            .collect::<Vec<_>>()
    };
    
    let tabs = Tabs::new(tab_titles)
        .select(app.active_tab_index)
        .style(Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::INACTIVE_TAB_BACKGROUND))
        .highlight_style(
            Style::default()
                .fg(VscodeTheme::TEXT)
                .bg(VscodeTheme::ACTIVE_TAB_BACKGROUND)
                .add_modifier(Modifier::BOLD)
        )
        .divider("│");
    
    f.render_widget(tabs, area);
}

/// Render file content with syntax highlighting
fn render_file_content(app: &mut App, f: &mut Frame, area: Rect) {
    // Use the syntax highlighter for more accurate VS Code-like highlighting (requires mutable app)
    let syntax_style = if let Some(path) = &app.current_file {
        app.syntax_highlighter.highlight_lines(&app.file_contents, Some(path))
    } else {
        // Fallback to basic styling if no file path is available
        crate::tui::syntax::get_default_syntax_styles(&app.file_contents)
    };
    
    let display_offset = app.editor_scroll_offset;
    
    // Calculate visible range
    let start_line = display_offset.min(app.file_contents.len().saturating_sub(1));
    let visible_height = area.height as usize;
    let end_line = (start_line + visible_height).min(app.file_contents.len());
    
    // Build spans for each visible line
    let mut line_spans = Vec::new();
    
    for i in start_line..end_line {
        let line_content = &app.file_contents[i];
        let is_current_line = app.code_view_state.selected_line.map_or(false, |line| line == i);
        let is_breakpoint = app.breakpoints.contains_key(&i);
        
        let line_num_style = if is_current_line {
            Style::default().fg(VscodeTheme::SELECTED_TEXT).bg(VscodeTheme::SELECTED_BACKGROUND)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        
        // Line number
        let line_number = Span::styled(format!("{:4} ", i + 1), line_num_style);
        
        // Breakpoint indicator
        let breakpoint_indicator = if is_breakpoint {
            Span::styled("● ", Style::default().fg(VscodeTheme::ERROR))
        } else {
            Span::styled("  ", Style::default())
        };
        
        // Apply syntax highlighting if available
        let line_content_span = if let Some(style) = syntax_style.get(&i) {
            Span::styled(line_content, *style)
        } else {
            Span::styled(line_content, Style::default().fg(VscodeTheme::TEXT))
        };
        
        // Build full line span
        let mut spans = vec![line_number.clone(), breakpoint_indicator.clone(), line_content_span];
        
        // Insert cursor if this is the current line and we're in edit mode
        if is_current_line && app.mode == AppMode::Edit {
            // Extract line up to cursor
            if let Some(pos) = app.code_view_state.cursor_position.checked_sub(1) {
                if pos < line_content.len() {
                    spans = vec![
                        line_number.clone(),
                        breakpoint_indicator.clone(),
                        Span::styled(&line_content[..pos], Style::default().fg(VscodeTheme::TEXT)),
                        Span::styled(&line_content[pos..=pos], Style::default().fg(VscodeTheme::TEXT).bg(Color::White)),
                        Span::styled(&line_content[pos+1..], Style::default().fg(VscodeTheme::TEXT))
                    ];
                }
            }
        }
        
        line_spans.push(Line::from(spans));
    }
    
    let file_content = Paragraph::new(line_spans)
        .style(Style::default().bg(VscodeTheme::EDITOR_BACKGROUND))
        .wrap(Wrap { trim: false });
    
    f.render_widget(file_content, area);
}

/// Render panel area with panel tabs and content
fn render_panel_area(app: &App, f: &mut Frame, area: Rect) {
    let panel_block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(VscodeTheme::BORDER))
        .style(Style::default().bg(VscodeTheme::PANEL_BACKGROUND));
    
    f.render_widget(panel_block, area);
    
    // Split panel area for tabs and content
    let panel_inner = area.inner(&Margin { horizontal: 0, vertical: 1 });
    let panel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),      // Panel tabs
            Constraint::Min(1),         // Panel content
        ])
        .split(panel_inner);
    
    let tabs_area = panel_layout[0];
    let content_area = panel_layout[1];
    
    // Panel tabs
    let tab_titles = vec![
        Line::from("TERMINAL"),
        Line::from("OUTPUT"),
        Line::from("PROBLEMS"),
        Line::from("DEBUG CONSOLE"),
    ];
    
    let selected_idx = match app.selected_panel_tab {
        PanelTab::Terminal => 0,
        PanelTab::Output => 1,
        PanelTab::Problems => 2,
        PanelTab::DebugConsole => 3,
        PanelTab::Custom(idx) => 4 + idx,
    };
    
    let tabs = Tabs::new(tab_titles)
        .select(selected_idx)
        .style(Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::PANEL_BACKGROUND))
        .highlight_style(
            Style::default()
                .fg(VscodeTheme::TEXT)
                .bg(VscodeTheme::ACTIVE_TAB_BACKGROUND)
                .add_modifier(Modifier::BOLD)
        )
        .divider("│");
    
    f.render_widget(tabs, tabs_area);
    
    // Panel content based on selected tab
    match app.selected_panel_tab {
        PanelTab::Terminal => render_terminal_panel(app, f, content_area),
        PanelTab::Output => render_output_panel(app, f, content_area),
        PanelTab::Problems => render_problems_panel(app, f, content_area),
        PanelTab::DebugConsole => render_debug_console_panel(app, f, content_area),
        PanelTab::Custom(idx) => {
            if idx < app.custom_panel_tabs.len() && idx < app.custom_panel_contents.len() {
                render_custom_panel(app, f, content_area, idx);
            }
        }
    }
}

/// Render terminal panel
fn render_terminal_panel(app: &App, f: &mut Frame, area: Rect) {
    // Split area to show terminal output and input
    let terminal_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),          // Terminal output
            Constraint::Length(1),       // Terminal input
        ])
        .split(area);
    
    let output_area = terminal_layout[0];
    let input_area = terminal_layout[1];
    
    // Terminal output - most recent at the bottom
    let visible_height = output_area.height as usize;
    let output_start = app.terminal_output.len().saturating_sub(visible_height);
    let visible_output: Vec<_> = app.terminal_output
        .iter()
        .skip(output_start)
        .map(|line| Line::from(line.clone()))
        .collect();
    
    let terminal_output = Paragraph::new(visible_output)
        .style(Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::PANEL_BACKGROUND));
    
    f.render_widget(terminal_output, output_area);
    
    // Terminal input
    let input_prefix = "$ ";
    let input_text = if app.terminal_input_active {
        format!("{}{}", input_prefix, app.terminal_input)
    } else {
        format!("{}{}", input_prefix, app.terminal_input)
    };
    
    let input_style = if app.terminal_input_active {
        Style::default().fg(VscodeTheme::TEXT).bg(Color::Rgb(45, 45, 45))
    } else {
        Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::PANEL_BACKGROUND)
    };
    
    let terminal_input = Paragraph::new(input_text)
        .style(input_style);
    
    f.render_widget(terminal_input, input_area);
}

/// Render output panel
fn render_output_panel(app: &App, f: &mut Frame, area: Rect) {
    let output_content = Paragraph::new(
        app.output_content
            .iter()
            .map(|line| Line::from(line.clone()))
            .collect::<Vec<_>>()
    )
    .style(Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::PANEL_BACKGROUND))
    .wrap(Wrap { trim: false });
    
    f.render_widget(output_content, area);
}

/// Render problems panel
fn render_problems_panel(app: &App, f: &mut Frame, area: Rect) {
    let problems_content = if app.problems.is_empty() {
        Paragraph::new("No problems detected in workspace")
            .style(Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::PANEL_BACKGROUND))
    } else {
        let problem_items: Vec<_> = app.problems
            .iter()
            .map(|problem| {
                let severity_style = match problem.severity.as_str() {
                    "error" => Style::default().fg(VscodeTheme::ERROR),
                    "warning" => Style::default().fg(VscodeTheme::WARNING),
                    _ => Style::default().fg(VscodeTheme::INFO),
                };
                
                let severity_icon = match problem.severity.as_str() {
                    "error" => "❌",
                    "warning" => "⚠️",
                    _ => "ℹ️",
                };
                
                let severity = Span::styled(format!("{} ", severity_icon), severity_style);
                let location = Span::styled(
                    format!("{}:{}:{} ", problem.file, problem.line, problem.column), 
                    Style::default().fg(VscodeTheme::TEXT)
                );
                let message = Span::styled(
                    problem.message.clone(),
                    Style::default().fg(VscodeTheme::TEXT)
                );
                
                Line::from(vec![severity, location, message])
            })
            .collect();
        
        Paragraph::new(problem_items)
            .style(Style::default().bg(VscodeTheme::PANEL_BACKGROUND))
            .wrap(Wrap { trim: false })
    };
    
    f.render_widget(problems_content, area);
}

/// Render debug console panel
fn render_debug_console_panel(app: &App, f: &mut Frame, area: Rect) {
    // Split area to show console output and input
    let console_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),          // Console output
            Constraint::Length(1),       // Console input
        ])
        .split(area);
    
    let output_area = console_layout[0];
    let input_area = console_layout[1];
    
    // Debug console output
    let debug_output = Paragraph::new(
        app.debug_console_output
            .iter()
            .map(|line| Line::from(line.clone()))
            .collect::<Vec<_>>()
    )
    .style(Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::PANEL_BACKGROUND))
    .wrap(Wrap { trim: false });
    
    f.render_widget(debug_output, output_area);
    
    // Debug console input
    let input_prefix = "> ";
    let input_text = if app.debug_console_input_active {
        format!("{}{}", input_prefix, app.debug_console_input)
    } else {
        format!("{}{}", input_prefix, app.debug_console_input)
    };
    
    let input_style = if app.debug_console_input_active {
        Style::default().fg(VscodeTheme::TEXT).bg(Color::Rgb(45, 45, 45))
    } else {
        Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::PANEL_BACKGROUND)
    };
    
    let debug_input = Paragraph::new(input_text)
        .style(input_style);
    f.render_widget(debug_input, input_area);
}

/// Render custom panel
fn render_custom_panel(app: &App, f: &mut Frame, area: Rect, idx: usize) {
    let custom_content = Paragraph::new(
        app.custom_panel_contents[idx]
            .iter()
            .map(|line| Line::from(line.clone()))
            .collect::<Vec<_>>()
    )
    .style(Style::default().fg(VscodeTheme::TEXT).bg(VscodeTheme::PANEL_BACKGROUND))
    .wrap(Wrap { trim: false });
    
    f.render_widget(custom_content, area);
}


/// Render command palette overlay
fn render_command_palette(app: &App, f: &mut Frame) {
    let size = f.size();
    
    // Create an overlay that's centered and takes up about 80% of the width and 50% of the height
    let width = std::cmp::min(80, size.width.saturating_sub(10));
    let height = std::cmp::min(20, size.height.saturating_sub(10));
    
    let h_padding = (size.width - width) / 2;
    let v_padding = (size.height - height) / 4; // Position it a bit higher than center
    
    let area = Rect::new(h_padding, v_padding, width, height);
    
    // Draw command palette
    let command_palette_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(VscodeTheme::BORDER))
        .style(Style::default().bg(VscodeTheme::SIDEBAR_BACKGROUND));
    
    f.render_widget(command_palette_block, area);
    
    // Inner area for content
    let inner_area = area.inner(&Margin { horizontal: 1, vertical: 1 });
    
    // Split for input and commands list
    let palette_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),      // Input
            Constraint::Length(1),      // Divider
            Constraint::Min(1),         // Commands list
        ])
        .split(inner_area);
    
    let input_area = palette_layout[0];
    let commands_area = palette_layout[2];
    
    // Input field
    let input_text = format!("> {}", app.command_palette_input);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(VscodeTheme::TEXT));
    
    f.render_widget(input, input_area);
    
    // Commands list
    // Filter commands based on command palette input
    let filtered_commands: Vec<String> = app.commands.iter()
        .filter(|cmd| {
            cmd.name.to_lowercase().contains(&app.command_palette_input.to_lowercase()) || 
            cmd.id.to_lowercase().contains(&app.command_palette_input.to_lowercase()) ||
            cmd.category.to_lowercase().contains(&app.command_palette_input.to_lowercase())
        })
        .map(|cmd| format!("{} ({})", cmd.name, cmd.id)).collect();
    let command_items: Vec<_> = filtered_commands
        .iter()
        .map(|cmd| {
            ListItem::new(cmd.clone())
                .style(Style::default().fg(VscodeTheme::TEXT))
        })
        .collect();
    
    let selected_command_idx = app.palette_selected_index.unwrap_or(0).min(filtered_commands.len().saturating_sub(1));
    
    let mut state = ListState::default();
    state.select(if filtered_commands.is_empty() { None } else { Some(selected_command_idx) });
    
    let commands_list = List::new(command_items)
        .block(Block::default())
        .highlight_style(
            Style::default()
                .bg(VscodeTheme::SELECTED_BACKGROUND)
                .add_modifier(Modifier::BOLD),
        );
    
    f.render_stateful_widget(commands_list, commands_area, &mut state);
}

/// Render dialog overlay
fn render_dialog(app: &App, f: &mut Frame) {
    if let Some(dialog) = &app.dialog {
        // Create a simple DialogInfo with the necessary fields based on DialogType
        let dialog_info = match dialog {
            super::app::DialogType::Confirm(msg, _) => {
                DialogInfo {
                    title: "Confirmation".to_string(),
                    message: msg.clone(),
                    buttons: vec!["Yes".to_string(), "No".to_string()],
                    active_button: Some(0),
                }
            },
            super::app::DialogType::Input(title, value, _) => {
                DialogInfo {
                    title: title.clone(),
                    message: format!("Current value: {}", value),
                    buttons: vec!["OK".to_string(), "Cancel".to_string()],
                    active_button: Some(0),
                }
            },
            super::app::DialogType::Message(msg) => DialogInfo {
                title: "Message".to_string(),
                message: msg.clone(),
                buttons: vec!["OK".to_string()],
                active_button: Some(0),
            },
        };
        
        // Dialog rendering
        
        let size = f.size();
        
        // Calculate dialog size
        let width = std::cmp::min(60, size.width.saturating_sub(10));
        let height = std::cmp::min(10, size.height.saturating_sub(10));
        
        let h_padding = (size.width - width) / 2;
        let v_padding = (size.height - height) / 2;
        
        let area = Rect::new(h_padding, v_padding, width, height);
        
        // Draw dialog
        let dialog_block = Block::default()
            .title(dialog_info.title.clone())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(VscodeTheme::BORDER))
            .style(Style::default().bg(VscodeTheme::SIDEBAR_BACKGROUND));
        
        f.render_widget(dialog_block, area);
        
        // Inner area for content
        let inner_area = area.inner(&Margin { horizontal: 2, vertical: 1 });
        
        // Split for message and buttons
        let dialog_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),      // Message
                Constraint::Length(1),   // Buttons
            ])
            .split(inner_area);
        
        let message_area = dialog_layout[0];
        let buttons_area = dialog_layout[1];
        
        // Message
        let message = Paragraph::new(dialog_info.message.clone())
            .style(Style::default().fg(VscodeTheme::TEXT))
            .wrap(Wrap { trim: true });
        
        f.render_widget(message, message_area);
        
        // Buttons
        let button_spans: Vec<_> = dialog_info.buttons
            .iter()
            .enumerate()
            .map(|(i, button)| {
                let is_active = dialog_info.active_button.map_or(false, |idx| idx == i);
                let style = if is_active {
                    Style::default().fg(VscodeTheme::SELECTED_TEXT).bg(VscodeTheme::STATUS_BAR_BACKGROUND)
                } else {
                    Style::default().fg(VscodeTheme::TEXT)
                };
                
                Span::styled(format!(" {} ", button), style)
            })
            .collect();
        
        let buttons = Paragraph::new(Line::from(button_spans))
            .style(Style::default().fg(VscodeTheme::TEXT));
        
        f.render_widget(buttons, buttons_area);
    }
}
    
/// Helper struct for dialog information
struct DialogInfo {
    /// Dialog title
    title: String,
    /// Dialog message
    message: String,
    /// Dialog buttons
    buttons: Vec<String>,
    /// Active button index
    active_button: Option<usize>,
}
