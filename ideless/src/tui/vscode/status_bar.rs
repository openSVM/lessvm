//! Status bar component for the VS Code-like TUI

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::Paragraph,
    Frame
};

use crate::tui::app::App;
use super::theme::VscodeTheme;
/// Render the status bar
pub fn render(app: &App, f: &mut Frame, area: Rect) {
    // Create layout for status items
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // Left status items
            Constraint::Percentage(30), // Right status items
        ])
        .split(area);
    
    let left_area = layout[0];
    let right_area = layout[1];
    
    // Left status items
    let mut left_items = Vec::new();
    
    // Git branch
    if let Some(branch) = &app.git_branch {
        left_items.push(
            StatusItem::new_key("git", &format!(" {} ", branch))
                .bg(VscodeTheme::STATUS_ACCENT_BG)
                .fg(VscodeTheme::STATUS_ACCENT_FG)
        );
    }
    
    // Current file info
    if let Some(current_file) = &app.current_file {
        left_items.push(
            StatusItem::new_key("file", &format!(" {} ", 
                current_file.file_name().unwrap_or_default().to_string_lossy()))
                .fg(VscodeTheme::STATUS_FOREGROUND)
        );
    }
    
    // Right status items
    let mut right_items = Vec::new();
    
    // Position info
    let line = app.code_view_state.selected_line.unwrap_or(0) + 1;
    let position = format!(" Ln {}, Col {} ", line, app.code_view_state.cursor_position + 1);
    right_items.push(
        StatusItem::new_key("position", &position)
            .fg(VscodeTheme::STATUS_FOREGROUND)
    );
    
    // Encoding info
    right_items.push(
        StatusItem::new_key("encoding", " UTF-8 ")
            .fg(VscodeTheme::STATUS_FOREGROUND)
    );
    
    // Render status items
    render_status_items(f, left_area, left_items, false);
    render_status_items(f, right_area, right_items, true);
}

/// Render a group of status items
fn render_status_items(f: &mut Frame, area: Rect, items: Vec<StatusItem>, align_right: bool) {
    let mut spans = Vec::new();
    
    // Create text spans for each status item
    for item in items {
        spans.push(
            Span::styled(
                item.text,
                Style::default()
                    .fg(item.fg_color)
                    .bg(item.bg_color)
                    .add_modifier(Modifier::BOLD)
            )
        );
    }

    // Create paragraph widget with all spans
    // Convert spans to a Line since Paragraph::new expects Text-convertible type
    let line = Line::from(spans);
    let paragraph = Paragraph::new(line)
        .alignment(if align_right { 
            tui::layout::Alignment::Right 
        } else { 
            tui::layout::Alignment::Left })
        .style(Style::default().bg(VscodeTheme::STATUS_BAR_BACKGROUND));
    
    f.render_widget(paragraph, area);
}

pub struct StatusItem {
    pub key: String,
    pub text: String,
    
    // Fields for styling
    pub fg_color: Color,
    pub bg_color: Color,
}

impl StatusItem {
    pub fn new(key: &str, text: String) -> Self {
        Self {
            key: key.to_string(),
            text,
            fg_color: VscodeTheme::STATUS_FOREGROUND,
            bg_color: VscodeTheme::STATUS_BAR_BACKGROUND,
        }
    }

    pub fn new_key(key: &str, text: &str) -> Self {
        Self {
            key: key.to_string(),
            text: text.to_string(),
            fg_color: VscodeTheme::STATUS_FOREGROUND,
            bg_color: VscodeTheme::STATUS_BAR_BACKGROUND,
        }
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg_color = color;
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }
}

