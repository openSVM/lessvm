//! Input event handling module
//!
//! This module handles input events for the TUI application.

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::{App, AppMode, AppResult, Tab, VMStatus};
use super::vscode::panel_area::PanelTab;
use super::vscode::activity_bar::ActivityView;

/// Handle key event for the application
pub fn handle_key_event(app: &mut App, key: KeyEvent) -> AppResult<()> {
    // Check for VS Code UI mode first
    if app.use_vscode_ui {
        return handle_vscode_key_event(app, key);
    }

    // Classic UI mode
    match app.mode {
        AppMode::Normal => handle_normal_mode(app, key),
        AppMode::Edit => handle_edit_mode(app, key), 
        AppMode::Command => handle_command_mode(app, key),
        AppMode::Dialog => handle_dialog_mode(app, key),
        AppMode::Help => handle_help_mode(app, key),
    }
}

/// Handle key event in normal mode
fn handle_normal_mode(app: &mut App, key: KeyEvent) -> AppResult<()> {
    match key.code {
        // Tab navigation
        KeyCode::Tab => {
            app.current_tab = match app.current_tab {
                Tab::Editor => Tab::Disassembler,
                Tab::Disassembler => Tab::VMState,
                Tab::VMState => Tab::Memory,
                Tab::Memory => Tab::Console,
                Tab::Console => Tab::Editor,
            };
            Ok(())
        },
        
        // Switch to numbered tabs
        KeyCode::Char('1') => { app.current_tab = Tab::Editor; Ok(()) },
        KeyCode::Char('2') => { app.current_tab = Tab::Disassembler; Ok(()) },
        KeyCode::Char('3') => { app.current_tab = Tab::VMState; Ok(()) },
        KeyCode::Char('4') => { app.current_tab = Tab::Memory; Ok(()) },
        KeyCode::Char('5') => { app.current_tab = Tab::Console; Ok(()) },
        
        // File operations
        KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // TODO: Open file dialog
            // For now, we'll just print a message to the console
            app.add_to_console("Open file not implemented yet".to_string());
            Ok(())
        },
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            match app.save_file() {
                Ok(_) => app.add_to_console("File saved successfully".to_string()),
                Err(e) => app.add_to_console(format!("Failed to save file: {}", e)),
            }
            Ok(())
        },
        
        // Switch to edit mode
        KeyCode::Char('i') => {
            if app.current_tab == Tab::Editor {
                app.mode = AppMode::Edit;
            }
            Ok(())
        },
        
        // Switch to command mode
        KeyCode::Char(':') => {
            app.mode = AppMode::Command;
            app.command_input.clear();
            Ok(())
        },
        
        // VM controls
        KeyCode::F(5) => {
            app.run_vm()
        },
        KeyCode::F(6) => {
            app.step_vm()
        },
        KeyCode::F(7) => {
            app.pause_vm()
        },
        KeyCode::F(8) => {
            app.stop_vm()
        },
        KeyCode::F(9) => {
            app.reset_vm()
        },
        
        // Toggle breakpoint
        KeyCode::Char('b') => {
            if app.current_tab == Tab::Editor {
                if let Some(line) = app.code_view_state.selected_line {
                    app.toggle_breakpoint(line);
                }
            }
            Ok(())
        },
        
        // Navigation
        KeyCode::Up | KeyCode::Char('k') => {
            navigate_up(app);
            Ok(())
        },
        KeyCode::Down | KeyCode::Char('j') => {
            navigate_down(app);
            Ok(())
        },
        KeyCode::Left | KeyCode::Char('h') => {
            navigate_left(app);
            Ok(())
        },
        KeyCode::Right | KeyCode::Char('l') => {
            navigate_right(app);
            Ok(())
        },
        
        // Page navigation
        KeyCode::PageUp => {
            navigate_page_up(app);
            Ok(())
        },
        KeyCode::PageDown => {
            navigate_page_down(app);
            Ok(())
        },
        KeyCode::Home => {
            navigate_home(app);
            Ok(())
        },
        KeyCode::End => {
            navigate_end(app);
            Ok(())
        },
        
        // Help
        KeyCode::F(1) | KeyCode::Char('?') => {
            app.mode = AppMode::Help;
            Ok(())
        },
        
        // Quit
        KeyCode::Char('q') => {
            app.running = false;
            Ok(())
        },
        KeyCode::Esc => {
            // If a dialog is showing, close it
            if app.dialog.is_some() {
                app.dialog = None;
                Ok(())
            } else {
                // Otherwise, ask for confirmation
                app.dialog = Some(super::app::DialogType::Confirm(
                    "Are you sure you want to quit? (y/n)".to_string(),
                    Box::new(|app| {
                        app.running = false;
                    }),
                ));
                Ok(())
            }
        },
        
        // Default
        _ => Ok(()),
    }
}

/// Handle key event in edit mode
fn handle_edit_mode(app: &mut App, key: KeyEvent) -> AppResult<()> {
    match key.code {
        // Exit edit mode
        KeyCode::Esc => {
            app.mode = AppMode::Normal;
            Ok(())
        },
        
        // Navigation
        KeyCode::Up => {
            navigate_up(app);
            Ok(())
        },
        KeyCode::Down => {
            navigate_down(app);
            Ok(())
        },
        KeyCode::Left => {
            // Move cursor left
            app.code_view_state.move_cursor_left();
            Ok(())
        },
        KeyCode::Right => {
            // Move cursor right
            app.code_view_state.move_cursor_right(&app.file_contents);
            Ok(())
        },
        
        // Text editing
        KeyCode::Backspace => {
            if let Some(line) = app.code_view_state.selected_line {
                if app.code_view_state.cursor_position > 0 {
                    if let Some(line_content) = app.file_contents.get_mut(line) {
                        let position = app.code_view_state.cursor_position;
                        if position <= line_content.len() as usize {
                            // Remove character at cursor position
                            line_content.remove(position - 1);
                            app.code_view_state.cursor_position -= 1;
                            app.modified = true;
                        }
                    }
                } else if line > 0 {
                    // Join with previous line
                    let current_line = app.file_contents.remove(line);
                    if let Some(prev_line) = app.file_contents.get_mut(line - 1) {
                        let prev_len = prev_line.len();
                        prev_line.push_str(&current_line);
                        app.code_view_state.selected_line = Some(line - 1);
                        app.code_view_state.cursor_position = prev_len;
                        app.modified = true;
                    }
                }
            }
            Ok(())
        },
        KeyCode::Delete => {
            if let Some(line) = app.code_view_state.selected_line {
                if let Some(line_content) = app.file_contents.get_mut(line) {
                    let position = app.code_view_state.cursor_position;
                    if position < line_content.len() as usize {
                        // Remove character at cursor position
                        line_content.remove(position);
                        app.modified = true;
                    } else if line < app.file_contents.len() - 1 {
                        // Join with next line
                        let next_line = app.file_contents.remove(line + 1);
                        if let Some(current_line) = app.file_contents.get_mut(line) {
                            current_line.push_str(next_line.as_str());
                            app.modified = true;
                        }
                    }
                }
            }
            Ok(())
        },
        KeyCode::Enter => {
            if let Some(line) = app.code_view_state.selected_line {
                if let Some(line_content) = app.file_contents.get(line).cloned() {
                    let position = app.code_view_state.cursor_position;
                    let (left, right) = line_content.split_at(position.min(line_content.len() as usize));
                    
                    // Update current line with left part
                    if let Some(current_line) = app.file_contents.get_mut(line) {
                        *current_line = left.to_string();
                    }
                    
                    // Insert new line with right part
                    app.file_contents.insert(line + 1, right.to_string());
                    
                    // Update cursor position
                    app.code_view_state.selected_line = Some(line + 1);
                    app.code_view_state.cursor_position = 0;
                    app.modified = true;
                }
            }
            Ok(())
        },
        KeyCode::Char(c) => {
            if let Some(line) = app.code_view_state.selected_line {
                if line >= app.file_contents.len() {
                    // Add new lines if needed
                    app.file_contents.resize(line + 1, String::new());
                }
                
                if let Some(line_content) = app.file_contents.get_mut(line) {
                    let position = app.code_view_state.cursor_position;
                    if position <= line_content.len() as usize {
                        // Insert character at cursor position
                        line_content.insert(position, c);
                        app.code_view_state.cursor_position += 1;
                        app.modified = true;
                    }
                }
            }
            Ok(())
        },
        KeyCode::Tab => {
            // Insert 4 spaces for tab
            if let Some(line) = app.code_view_state.selected_line {
                if let Some(line_content) = app.file_contents.get_mut(line) {
                    let position = app.code_view_state.cursor_position;
                    if position <= line_content.len() as usize {
                        // Insert tab at cursor position
                        line_content.insert_str(position, "    ");
                        app.code_view_state.cursor_position += 4;
                        app.modified = true;
                    }
                }
            }
            Ok(())
        },
        
        // Default
        _ => Ok(()),
    }
}

/// Handle key event in command mode
fn handle_command_mode(app: &mut App, key: KeyEvent) -> AppResult<()> {
    match key.code {
        // Exit command mode
        KeyCode::Esc => {
            app.mode = AppMode::Normal;
            app.command_input.clear();
            Ok(())
        },
        
        // Execute command
        KeyCode::Enter => {
            let command = app.command_input.clone();
            app.command_input.clear();
            app.mode = AppMode::Normal;
            app.execute_command(&command)
        },
        
        // Command history navigation
        KeyCode::Up => {
            if !app.command_history.is_empty() {
                if app.command_history_index > 0 {
                    app.command_history_index -= 1;
                }
                if let Some(cmd) = app.command_history.get(app.command_history_index) {
                    app.command_input = cmd.clone();
                }
            }
            Ok(())
        },
        KeyCode::Down => {
            if !app.command_history.is_empty() {
                if app.command_history_index < app.command_history.len() - 1 {
                    app.command_history_index += 1;
                    if let Some(cmd) = app.command_history.get(app.command_history_index) {
                        app.command_input = cmd.clone();
                    }
                } else {
                    app.command_history_index = app.command_history.len();
                    app.command_input.clear();
                }
            }
            Ok(())
        },
        
        // Editing
        KeyCode::Backspace => {
            app.command_input.pop();
            Ok(())
        },
        KeyCode::Char(c) => {
            app.command_input.push(c);
            Ok(())
        },
        
        // Default
        _ => Ok(()),
    }
}

/// Handle key event in dialog mode
fn handle_dialog_mode(app: &mut App, key: KeyEvent) -> AppResult<()> {
    // Handle based on dialog type
    if app.dialog.is_none() {
        // No dialog active, switch back to normal mode
        app.mode = AppMode::Normal;
        return Ok(());
    }
    
    // Extract dialog type
    let dialog_type = match &app.dialog {
        Some(super::app::DialogType::Confirm(_, _)) => 0,
        Some(super::app::DialogType::Input(_, _, _)) => 1,
        Some(super::app::DialogType::Message(_)) => 2,
        None => return Ok(()),
    };
    
    // Handle based on dialog type
    match dialog_type {
        // Confirm dialog
        0 => match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                // Take dialog out of app to avoid borrow issues
                if let Some(super::app::DialogType::Confirm(_msg, callback)) = app.dialog.take() {
                    callback(app);
                }
                Ok(())
            },
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                app.dialog = None;
                Ok(())
            },
            _ => Ok(()),
        },
        
        // Input dialog
        1 => match key.code {
            KeyCode::Enter => {
                // Take dialog out of app to avoid borrow issues
                if let Some(super::app::DialogType::Input(_title, value, callback)) = app.dialog.take() {
                    callback(app, &value);
                }
                Ok(())
            },
            KeyCode::Esc => {
                app.dialog = None;
                Ok(())
            },
            KeyCode::Backspace => {
                if let Some(super::app::DialogType::Input(_title, value, _callback)) = &mut app.dialog {
                    value.pop();
                }
                Ok(())
            },
            KeyCode::Char(c) => {
                if let Some(super::app::DialogType::Input(_title, value, _callback)) = &mut app.dialog {
                    value.push(c);
                }
                Ok(())
            },
            _ => Ok(()),
        },
        
        // Message dialog
        2 => match key.code {
            KeyCode::Enter | KeyCode::Esc => {
                app.dialog = None;
                Ok(())
            },
            _ => Ok(()),
        }
        
        // Should never happen
        _ => Ok(()),
    }
}

/// Handle key event in help mode
fn handle_help_mode(app: &mut App, key: KeyEvent) -> AppResult<()> {
    match key.code {
        KeyCode::Esc | KeyCode::F(1) | KeyCode::Char('?') | KeyCode::Char('q') => {
            app.mode = AppMode::Normal;
            Ok(())
        },
        KeyCode::Up | KeyCode::Char('k') => {
            // Scroll help up
            // In a real implementation, we would scroll the help text
            Ok(())
        },
        KeyCode::Down | KeyCode::Char('j') => {
            // Scroll help down
            // In a real implementation, we would scroll the help text
            Ok(())
        },
        _ => Ok(()),
    }
}

// Navigation helper functions

/// Navigate up
fn navigate_up(app: &mut App) {
    match app.current_tab {
        Tab::Editor => {
            if let Some(line) = app.code_view_state.selected_line {
                if line > 0 {
                    app.code_view_state.selected_line = Some(line - 1);
                    
                    // Adjust cursor position based on line length
                    if let Some(line_content) = app.file_contents.get(line - 1) {
                        app.code_view_state.cursor_position = app.code_view_state.cursor_position.min(line_content.len() as usize);
                    }
                    
                    // Adjust scroll if needed
                    if line - 1 < app.code_view_state.offset {
                        app.code_view_state.offset = line - 1;
                    }
                }
            } else {
                app.code_view_state.selected_line = Some(0);
                app.code_view_state.cursor_position = 0;
            }
        },
        Tab::Console => {
            if let Some(selected) = app.console_list_state.selected() {
                if selected > 0 {
                    app.console_list_state.select(Some(selected - 1));
                }
            }
        },
        _ => {}
    }
}

/// Handle key events for VS Code-like UI
fn handle_vscode_key_event(app: &mut App, key: KeyEvent) -> AppResult<()> {
    // Handle dialog mode first if active
    if app.dialog.is_some() {
        return handle_dialog_mode(app, key);
    }

    // Handle command palette if visible
    if app.command_palette_visible {
        return handle_command_palette_mode(app, key);
    }

    // Handle edit mode if active
    if app.mode == AppMode::Edit {
        // Use the existing edit mode handler with some VS Code-specific overrides
        return handle_vscode_edit_mode(app, key);
    }

    // Normal VS Code navigation and commands
    match key.code {
        // Toggle sidebar (Ctrl+B)
        KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.toggle_sidebar();
            Ok(())
        },

        // Toggle panel (Ctrl+J)
        KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.toggle_panel();
            Ok(())
        },

        // Command palette (Ctrl+P or F1)
        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.toggle_command_palette();
            Ok(())
        },
        KeyCode::F(1) => {
            app.toggle_command_palette();
            Ok(())
        },

        // Activity bar views
        // Explorer (Ctrl+Shift+E)
        KeyCode::Char('E') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
            app.switch_activity_view(ActivityView::Explorer);
            Ok(())
        },
        // Search (Ctrl+Shift+F)
        KeyCode::Char('F') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
            app.switch_activity_view(ActivityView::Search);
            Ok(())
        },
        // Source Control (Ctrl+Shift+G)
        KeyCode::Char('G') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
            app.switch_activity_view(ActivityView::SourceControl);
            Ok(())
        },
        // Debug (Ctrl+Shift+D)
        KeyCode::Char('D') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
            app.switch_activity_view(ActivityView::Debug);
            Ok(())
        },
        // Extensions (Ctrl+Shift+X)
        KeyCode::Char('X') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
            app.switch_activity_view(ActivityView::Extensions);
            Ok(())
        },

        // File operations
        // Open file (Ctrl+O)
        KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.add_to_console("Open file dialog would appear here".to_string());
            Ok(())
        },
        // New file (Ctrl+N)
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.add_to_console("New file created".to_string());
            // Execute command to create new file
            app.execute_command_by_id("file.new")
        },
        // Save file (Ctrl+S)
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.save_file()
        },

        // Panel tabs
        // Terminal (Ctrl+`)
        KeyCode::Char('`') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if !app.panel_visible {
                app.toggle_panel();
            }
            app.switch_panel_tab(PanelTab::Terminal);
            app.terminal_input_active = true;
            Ok(())
        },

        // Toggle between editor tabs
        KeyCode::Tab if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Cycle to next tab
            if !app.open_files.is_empty() {
                app.active_tab_index = (app.active_tab_index + 1) % app.open_files.len();
            }
            Ok(())
        },

        // Toggle UI style (custom shortcut)
        KeyCode::F(12) => {
            app.toggle_ui_style();
            Ok(())
        },

        // Other keyboard navigation
        _ => handle_normal_mode(app, key),
    }
}

/// Handle key events for VS Code-like command palette mode
fn handle_command_palette_mode(app: &mut App, key: KeyEvent) -> AppResult<()> {
    match key.code {
        // Close command palette
        KeyCode::Esc => {
            app.toggle_command_palette();
            Ok(())
        },
        
        // Execute selected command
        KeyCode::Enter => {
            if let Some(idx) = app.palette_selected_index {
                if idx < app.command_palette_items.len() {
                    let command_id = app.command_palette_items[idx].id.clone();
                    app.toggle_command_palette();
                    app.execute_command_by_id(&command_id)
                } else {
                    Ok(())
                }
            } else {
                app.toggle_command_palette();
                Ok(())
            }
        },
        
        // Navigate up/down in the command palette
        KeyCode::Up => {
            if !app.command_palette_items.is_empty() {
                app.palette_selected_index = Some(
                    app.palette_selected_index
                        .map(|i| if i > 0 { i - 1 } else { app.command_palette_items.len() - 1 })
                        .unwrap_or(app.command_palette_items.len() - 1)
                );
            }
            Ok(())
        },
        KeyCode::Down => {
            if !app.command_palette_items.is_empty() {
                app.palette_selected_index = Some(
                    app.palette_selected_index
                        .map(|i| (i + 1) % app.command_palette_items.len())
                        .unwrap_or(0)
                );
            }
            Ok(())
        },
        
        _ => Ok(()),
    }
}

/// Handle key events for VS Code-like edit mode
fn handle_vscode_edit_mode(app: &mut App, key: KeyEvent) -> AppResult<()> {
    match key.code {
        // VS Code specific edit mode overrides
        KeyCode::Esc => {
            app.mode = AppMode::Normal;
            Ok(())
        },
        
        // Use the existing edit mode handler for text editing
        _ => handle_edit_mode(app, key),
    }
}

/// Navigate down
fn navigate_down(app: &mut App) {
    match app.current_tab {
        Tab::Editor => {
            let max_line = app.file_contents.len().saturating_sub(1);
            if let Some(line) = app.code_view_state.selected_line {
                if line < max_line {
                    app.code_view_state.selected_line = Some(line + 1);
                    
                    // Adjust cursor position based on line length
                    if let Some(line_content) = app.file_contents.get(line + 1) {
                        app.code_view_state.cursor_position = app.code_view_state.cursor_position.min(line_content.len() as usize);
                    }
                    
                    // Adjust scroll if needed (assuming 20 visible lines)
                    if line + 1 >= app.code_view_state.offset + 20 {
                        app.code_view_state.offset = line + 1 - 19;
                    }
                }
            } else if !app.file_contents.is_empty() {
                app.code_view_state.selected_line = Some(0);
                app.code_view_state.cursor_position = 0;
            }
        },
        Tab::Console => {
            if let Some(selected) = app.console_list_state.selected() {
                if selected < app.console_history.len().saturating_sub(1) {
                    app.console_list_state.select(Some(selected + 1));
                }
            }
        },
        _ => {}
    }
}

/// Navigate left
fn navigate_left(app: &mut App) {
    match app.current_tab {
        Tab::Memory => {
            app.memory_view_state.selected_address = app.memory_view_state.selected_address
                .map(|addr| addr.saturating_sub(1));
        },
        // Add more tabs as needed
        _ => {}
    }
}

/// Navigate right
fn navigate_right(app: &mut App) {
    match app.current_tab {
        Tab::Memory => {
            if let Some(addr) = app.memory_view_state.selected_address {
                let max_addr = app.vm.get_memory().len() - 1;
                if addr < max_addr {
                    app.memory_view_state.selected_address = Some(addr + 1);
                }
            } else {
                app.memory_view_state.selected_address = Some(0);
            }
        },
        // Add more tabs as needed
        _ => {}
    }
}

/// Navigate page up
fn navigate_page_up(app: &mut App) {
    match app.current_tab {
        Tab::Editor => {
            // Assume 20 lines per page
            if let Some(line) = app.code_view_state.selected_line {
                let new_line = line.saturating_sub(20);
                app.code_view_state.selected_line = Some(new_line);
                
                // Adjust cursor position based on line length
                if let Some(line_content) = app.file_contents.get(new_line) {
                    app.code_view_state.cursor_position = app.code_view_state.cursor_position.min(line_content.len() as usize);
                }
                
                // Adjust scroll if needed
                if new_line < app.code_view_state.offset {
                    app.code_view_state.offset = new_line;
                }
            }
        },
        Tab::Console => {
            if let Some(selected) = app.console_list_state.selected() {
                let new_selected = selected.saturating_sub(10);
                app.console_list_state.select(Some(new_selected));
            }
        },
        Tab::Memory => {
            app.memory_view_state.selected_address = app.memory_view_state.selected_address
                .map(|addr| addr.saturating_sub(16 * 10));
        },
        _ => {}
    }
}

/// Navigate page down
fn navigate_page_down(app: &mut App) {
    match app.current_tab {
        Tab::Editor => {
            // Assume 20 lines per page
            let max_line = app.file_contents.len().saturating_sub(1);
            if let Some(line) = app.code_view_state.selected_line {
                let new_line = (line + 20).min(max_line);
                app.code_view_state.selected_line = Some(new_line);
                
                // Adjust cursor position based on line length
                if let Some(line_content) = app.file_contents.get(new_line) {
                    app.code_view_state.cursor_position = app.code_view_state.cursor_position.min(line_content.len() as usize);
                }
                
                // Adjust scroll if needed
                if new_line >= app.code_view_state.offset + 20 {
                    app.code_view_state.offset = new_line - 19;
                }
            }
        },
        Tab::Console => {
            if let Some(selected) = app.console_list_state.selected() {
                let new_selected = (selected + 10).min(app.console_history.len().saturating_sub(1));
                app.console_list_state.select(Some(new_selected));
            }
        },
        Tab::Memory => {
            if let Some(addr) = app.memory_view_state.selected_address {
                let max_addr = app.vm.get_memory().len() - 1;
                let new_addr = (addr + 16 * 10).min(max_addr);
                app.memory_view_state.selected_address = Some(new_addr);
            }
        },
        _ => {}
    }
}

/// Navigate home
fn navigate_home(app: &mut App) {
    match app.current_tab {
        Tab::Editor => {
            if app.code_view_state.selected_line.is_some() {
                app.code_view_state.cursor_position = 0;
            }
        },
        Tab::Console => {
            app.console_list_state.select(Some(0));
        },
        Tab::Memory => {
            app.memory_view_state.selected_address = Some(0);
        },
        _ => {}
    }
}

/// Navigate end
fn navigate_end(app: &mut App) {
    match app.current_tab {
        Tab::Editor => {
            if let Some(line) = app.code_view_state.selected_line {
                if let Some(line_content) = app.file_contents.get(line) {
                    app.code_view_state.cursor_position = line_content.len() as usize;
                }
            }
        },
        Tab::Console => {
            let last = app.console_history.len().saturating_sub(1);
            app.console_list_state.select(Some(last));
        },
        Tab::Memory => {
            let max_addr = app.vm.get_memory().len() - 1;
            app.memory_view_state.selected_address = Some(max_addr);
        },
        _ => {}
    }
}