//! Command palette component for the VS Code-like TUI

use tui::{
    backend::Backend,
    Frame
};

use crate::tui::app::App;

/// Command for the command palette
#[derive(Debug, Clone)]
pub struct Command {
    /// Command ID
    pub id: String,
    /// Command name
    pub name: String,
    /// Command category
    pub category: String,
    /// Keybinding (optional)
    pub keybinding: Option<String>,
}

impl Command {
    /// Create a new command
    pub fn new(id: &str, name: &str, category: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            category: category.to_string(),
            keybinding: None,
        }
    }
    
    /// Add keybinding to command
    pub fn keybinding(mut self, kb: &str) -> Self {
        self.keybinding = Some(kb.to_string());
        self
    }
}

/// Render the command palette
pub fn render(_app: &App, _f: &mut Frame, _area: tui::layout::Rect) {
    
}