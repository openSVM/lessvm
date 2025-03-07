//! Theme management for the VS Code-like TUI

use tui::style::Color;

/// VS Code-like theme
pub struct VscodeTheme;
/// Color aliases for theme components
pub type ThemeColor = Color;

impl VscodeTheme {
    /// Background color
    pub const BACKGROUND: Color = Color::Rgb(30, 30, 30);
    /// Activity bar background
    pub const ACTIVITY_BAR_BACKGROUND: Color = Color::Rgb(51, 51, 51);
    /// Sidebar background
    pub const SIDEBAR_BACKGROUND: Color = Color::Rgb(37, 37, 38);
    /// Editor background
    pub const EDITOR_BACKGROUND: Color = Color::Rgb(30, 30, 30);
    /// Panel background
    pub const PANEL_BACKGROUND: Color = Color::Rgb(37, 37, 38);
    /// Status bar background
    pub const STATUS_BAR_BACKGROUND: Color = Color::Rgb(0, 122, 204);
    /// Status bar foreground color
    pub const STATUS_FOREGROUND: Color = Color::Rgb(255, 255, 255);
    /// Status bar accent background color
    pub const STATUS_ACCENT_BG: Color = Color::Rgb(36, 114, 200);
    /// Status bar accent foreground color
    pub const STATUS_ACCENT_FG: Color = Color::Rgb(255, 255, 255);
    /// Active tab background
    pub const ACTIVE_TAB_BACKGROUND: Color = Color::Rgb(37, 37, 38);
    /// Inactive tab background
    pub const INACTIVE_TAB_BACKGROUND: Color = Color::Rgb(45, 45, 45);
    /// Border color
    pub const BORDER: Color = Color::Rgb(60, 60, 60);
    /// Text color
    pub const TEXT: Color = Color::Rgb(212, 212, 212);
    /// Comment color
    pub const COMMENT: Color = Color::Rgb(106, 153, 85);
    /// String color
    pub const STRING: Color = Color::Rgb(206, 145, 120);
    /// Keyword color
    pub const KEYWORD: Color = Color::Rgb(86, 156, 214);
    /// Function color
    pub const FUNCTION: Color = Color::Rgb(220, 220, 170);
    /// Number color
    pub const NUMBER: Color = Color::Rgb(181, 206, 168);
    /// Type color
    pub const TYPE: Color = Color::Rgb(78, 201, 176);
    /// Error color
    pub const ERROR: Color = Color::Rgb(244, 71, 71);
    /// Warning color
    pub const WARNING: Color = Color::Rgb(255, 213, 0);
    /// Info color
    pub const INFO: Color = Color::Rgb(23, 184, 239);
    /// Git added color
    pub const GIT_ADDED: Color = Color::Rgb(127, 186, 0);
    /// Git modified color
    pub const GIT_MODIFIED: Color = Color::Rgb(0, 122, 204);
    /// Git deleted color
    pub const GIT_DELETED: Color = Color::Rgb(244, 71, 71);
    /// Selected item background
    pub const SELECTED_BACKGROUND: Color = Color::Rgb(37, 37, 38);
    /// Selected item text color
    pub const SELECTED_TEXT: Color = Color::White;
}