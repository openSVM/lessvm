//! Syntax highlighting module
//!
//! Provides syntax highlighting functionality using the syntect library,
//! matching VS Code's highlighting as closely as possible in a terminal.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
}; 
use tui::style::{Color, Modifier, Style};
use syntect::{
    easy::HighlightLines,
    highlighting::{Theme, ThemeSet},
    parsing::{SyntaxDefinition, SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
};

use crate::tui::vscode::theme::VscodeTheme;

/// Manages syntax highlighting resources
pub struct SyntaxHighlighter {
    /// Syntax set containing all loaded syntaxes
    syntax_set: SyntaxSet,
    /// Theme set containing all loaded themes
    theme_set: ThemeSet,
    /// Current active theme
    current_theme: String,
    /// Cache for file extensions to syntax
    extension_cache: HashMap<String, Arc<SyntaxReference>>,
}

impl SyntaxHighlighter {
    /// Create a new syntax highlighter with default settings
    pub fn new() -> Self {
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        
        Self {
            syntax_set: ss,
            theme_set: ts,
            current_theme: "Monokai".to_string(), // Default theme similar to VS Code dark+
            extension_cache: HashMap::new(),
        }
    }
    
    /// Set the current theme
    pub fn set_theme(&mut self, theme_name: &str) -> Result<(), String> {
        if self.theme_set.themes.contains_key(theme_name) {
            self.current_theme = theme_name.to_string();
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", theme_name))
        }
    }
    
    /// Get a reference to the current theme
    fn current_theme(&self) -> &Theme {
        self.theme_set.themes.get(&self.current_theme).unwrap()
    }
    
    /// Get the syntax for a file by its path
    fn syntax_for_file(&mut self, path: &Path) -> Option<Arc<SyntaxReference>> {
        // Get file extension
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            // Check cache first
            if let Some(syntax) = self.extension_cache.get(ext) {
                return Some(Arc::clone(syntax));
            }
            
            // Try to find syntax by extension
            if let Some(syntax) = self.syntax_set.find_syntax_by_extension(ext) {
                let syntax_ref = Arc::new(syntax.clone());
                self.extension_cache.insert(ext.to_string(), Arc::clone(&syntax_ref));
                return Some(syntax_ref);
            }
        }
        
        // Fallback to first line detection
        None
    }
    
    /// Get the syntax for contents based on the first line
    fn syntax_for_first_line(&self, first_line: &str) -> Option<Arc<SyntaxReference>> {
        self.syntax_set.find_syntax_by_first_line(first_line)
            .map(|s| Arc::new(s.clone()))
    }
    
    /// Highlight lines of code with syntax highlighting
    pub fn highlight_lines(&mut self, content: &[String], file_path: Option<&Path>) -> HashMap<usize, Style> {
        let mut styles = HashMap::new();
        
        // Early return if empty content
        if content.is_empty() {
            return styles;
        }
        
        // Determine syntax
        let syntax = if let Some(path) = file_path {
            self.syntax_for_file(path)
        } else {
            self.syntax_for_first_line(&content[0])
        }.unwrap_or_else(|| Arc::new(self.syntax_set.find_syntax_plain_text().clone()));
        
        let theme = self.current_theme();
        let mut highlighter = HighlightLines::new(&syntax, theme);
        
        // Create a joined text for proper highlighting across line boundaries
        let joined_text = content.join("\n");
        
        // Map syntect colors to Ratatui colors
        for (i, line) in LinesWithEndings::from(&joined_text).enumerate() {
            if i >= content.len() {
                break;
            }
            
            if let Ok(ranges) = highlighter.highlight_line(line, &self.syntax_set) {
                // Get most prominent style for the line (simple approach)
                // In a real implementation, we would map each character range to a span
                // For now, just use a simple color based on line content
                if !line.trim().is_empty() {
                    // Use a default style based on the theme
                    let tui_style = if line.contains("fn ") || line.contains("function") {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    styles.insert(i, tui_style);
                }
            }
        }
        
        styles
    }
    
    /// Get language name for a file path
    pub fn language_for_file(&mut self, path: &Path) -> String {
        if let Some(syntax) = self.syntax_for_file(path) {
            syntax.name.clone()
        } else {
            "Plain Text".to_string()
        }
    }
    
    /// Get available languages
    pub fn available_languages(&self) -> Vec<String> {
        self.syntax_set.syntaxes()
            .iter()
            .map(|s| s.name.clone())
            .collect()
    }
    
    /// Get available themes
    pub fn available_themes(&self) -> Vec<String> {
        self.theme_set.themes.keys()
            .map(|k| k.clone())
            .collect()
    }
}

/// Convert syntect color to ratatui color
fn syntect_to_ratatui_color(color: syntect::highlighting::Color) -> Color {
    Color::Rgb(color.r, color.g, color.b)
}

/// Provide default styles for common code elements when syntax highlighting is unavailable
pub fn get_default_syntax_styles(content: &[String]) -> HashMap<usize, Style> {
    let mut styles = HashMap::new();
    
    for (i, line) in content.iter().enumerate() {
        let line = line.trim();
        
        if line.starts_with("//") || line.starts_with(";") || line.starts_with("#") {
            // Comment
            styles.insert(i, Style::default().fg(VscodeTheme::COMMENT));
        } else if line.contains("\"") || line.contains("'") {
            // Line with string
            styles.insert(i, Style::default().fg(VscodeTheme::STRING));
        } else if line.starts_with("fn ") || line.starts_with("function ") || line.contains("->") {
            // Function definition
            styles.insert(i, Style::default().fg(VscodeTheme::FUNCTION));
        } else if line.contains("struct ") || line.contains("class ") || line.contains("interface ") {
            // Type definition
            styles.insert(i, Style::default().fg(VscodeTheme::TYPE));
        } else if line.contains("let ") || line.contains("var ") || line.contains("const ") {
            // Variable definition
            styles.insert(i, Style::default().fg(VscodeTheme::KEYWORD));
        } else if line.contains("if ") || line.contains("else ") || line.contains("for ") || line.contains("while ") {
            // Control flow
            styles.insert(i, Style::default().fg(VscodeTheme::KEYWORD));
        } else if line.chars().all(|c| c.is_digit(10) || c.is_whitespace() || c == ',' || c == '.' || c == 'x' || c == 'b' || c == '0') {
            // Numeric line
            styles.insert(i, Style::default().fg(VscodeTheme::NUMBER));
        } else {
            // Default
            styles.insert(i, Style::default().fg(VscodeTheme::TEXT));
        }
    }
    
    styles
}