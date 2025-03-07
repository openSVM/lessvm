//! Keyboard shortcuts for the VS Code-like TUI

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::App;

/// Handle key events in the VS Code-like UI
pub fn handle_key_event(_app: &mut App, _key: KeyEvent) {
    // TODO: Implement custom keybindings and a way to configure them
    // For now, we'll use the default VS Code keybindings handled in `event.rs`
}

/// Check if a key event matches a specific keybinding
pub fn match_key_event(key: KeyEvent, code: KeyCode, modifiers: KeyModifiers) -> bool {
    key.code == code && key.modifiers == modifiers
}