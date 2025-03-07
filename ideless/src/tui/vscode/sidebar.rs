//! Sidebar component for the VS Code-like TUI

use tui::{
    layout::{Layout, Direction, Constraint, Rect, Margin},
    backend::Backend,
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
    Frame,
};

use crate::tui::{app::App, vscode::theme::VscodeTheme, vscode::activity_bar::ActivityView};

/// Render sidebar with content based on active view
pub fn render(app: &App, f: &mut Frame, area: Rect) {
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
    // TODO: Implement content rendering based on the active view
}
