//! Panel area component for the VS Code-like TUI

use tui::{
    backend::Backend,
    Frame
};

use crate::tui::app::App;

/// Panel tab types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelTab {
    /// Terminal tab
    Terminal,
    /// Output tab
    Output,
    /// Problems tab
    Problems,
    /// Debug console tab
    DebugConsole,
    /// Custom tab with index
    Custom(usize),
}

/// Render the panel area
pub fn render(_app: &App, _f: &mut Frame, _area: tui::layout::Rect) {
    
}