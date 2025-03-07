//! Activity Bar component for the VS Code-like TUI

use tui::{
    layout::Rect,
    backend::Backend,
    widgets::{Block, Borders, List, ListItem},
    style::{Style, Modifier, Color},
    text::Span,
    Frame,
};

use crate::tui::{app::App, vscode::theme::VscodeTheme};

/// Render the activity bar
pub fn render_activity_bar(app: &App, f: &mut Frame, area: Rect) {
    let activity_items = vec![
        ("📁", "Explorer", ActivityView::Explorer),
        ("🔍", "Search", ActivityView::Search),
        ("🔀", "Source Control", ActivityView::SourceControl),
        ("▶", "Debug", ActivityView::Debug),
        ("⚙", "Extensions", ActivityView::Extensions),
    ];

    let block = Block::default()
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(VscodeTheme::BORDER))
        .style(Style::default().bg(VscodeTheme::ACTIVITY_BAR_BACKGROUND));

    f.render_widget(block, area);

    let inner_area = Rect::new(area.x, area.y, area.width, area.height);

    let activity_spans: Vec<ListItem> = activity_items
        .iter()
        .map(|(icon, _label, view)| {
            let selected = app.active_view.map_or(false, |active| active == *view);
            let style = if selected {
                Style::default()
                    .fg(VscodeTheme::SELECTED_TEXT)
                    .bg(VscodeTheme::ACTIVITY_BAR_BACKGROUND)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(VscodeTheme::TEXT)
                    .bg(VscodeTheme::ACTIVITY_BAR_BACKGROUND)
            };

            let text = if selected {
                format!(" {} ", icon)
            } else {
                format!(" {} ", icon)
            };

            ListItem::new(text).style(style)
        })
        .collect();

    let list = List::new(activity_spans)
        .block(Block::default().style(Style::default().bg(VscodeTheme::ACTIVITY_BAR_BACKGROUND)))
        .highlight_style(
            Style::default()
                .bg(VscodeTheme::SELECTED_BACKGROUND)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(list, inner_area);
}

/// Enum representing the different views in the activity bar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ActivityView {
    Explorer,
    Search,
    SourceControl,
    Debug,
    Extensions,
}