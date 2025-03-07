//! UI widgets and state structures for the TUI IDE
//! 
//! This module contains reusable widgets and state structures for the TUI interface.

use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

/// Code editor view state
#[derive(Debug, Clone)]
pub struct CodeViewState {
    /// Selected line in the editor (cursor position vertical)
    pub selected_line: Option<usize>,
    
    /// Cursor position within the line (column/horizontal)
    pub cursor_position: usize,
    
    /// Scroll offset (first visible line)
    pub offset: usize,
    
    /// Search query
    pub search_query: Option<String>,
    
    /// Search results (line numbers)
    pub search_results: Vec<usize>,
    
    /// Current search result index
    pub current_search_idx: usize,
}

impl CodeViewState {
    /// Create a new code view state
    pub fn new() -> Self {
        Self {
            selected_line: Some(0),
            cursor_position: 0,
            offset: 0,
            search_query: None,
            search_results: Vec::new(),
            current_search_idx: 0,
        }
    }

    /// Move cursor left one position, ensuring it doesn't go beyond the start of the line
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// Move cursor right one position, ensuring it doesn't go beyond the end of the line
    pub fn move_cursor_right(&mut self, file_contents: &[String]) {
        if let Some(line_idx) = self.selected_line {
            if line_idx < file_contents.len() {
                let line = &file_contents[line_idx];
                // Allow cursor to be at the end of the line (for insertion)
                if self.cursor_position < line.len() {
                    self.cursor_position += 1;
                }
            }
        }
    }
    
    // Add more methods as needed for the CodeViewState
}

/// Memory viewer state
#[derive(Debug, Clone)]
pub struct MemoryViewState {
    /// Current address being viewed
    pub address: usize,
    
    /// Selected address (cursor position)
    pub selected_address: Option<usize>,
    
    /// Bytes per line
    pub bytes_per_line: usize,
    
    /// Show ASCII representation
    pub show_ascii: bool,
    
    pub changed_addresses: Vec<usize>,
}

impl MemoryViewState {
    /// Create a new memory view state
    pub fn new() -> Self {
        Self {
            address: 0,
            selected_address: Some(0),
            bytes_per_line: 16,
            show_ascii: true,
            changed_addresses: Vec::new(),
        }
    }
    
    /// Check if the given address has recently changed
    pub fn has_address_changed(&self, addr: usize) -> bool {
        self.changed_addresses.contains(&addr)
    }
    
    /// Format a byte as ASCII, replacing non-printable characters with dots
    pub fn format_byte_as_ascii(&self, byte: u8) -> char {
        if byte >= 32 && byte <= 126 {
            byte as char
        } else {
            '.'
        }
    }
}

impl DisassemblyView<'_> {
    /// Check if the given line corresponds to the current instruction
    pub fn is_current_instruction(line_idx: usize, pc: usize) -> bool {
        line_idx == pc
    }
    
    /// Format a disassembly line with line number
    pub fn format_line(line_idx: usize, line: &str) -> String {
        format!("{:04X}: {}", line_idx, line)
    }
}

impl RegisterView<'_> {
    /// Format a register name and value
    pub fn format_register(name: &str, value: &str) -> String {
        format!("{}: {}", name, value)
    }
}

impl StackView<'_> {
    /// Format a stack item with index
    pub fn format_stack_item(item_idx: usize, value: &str) -> String {
        let prefix = if item_idx == 0 {
            "SP→ "
        } else {
            "    "
        };
        
        format!("{}{:04X}: {}", prefix, item_idx, value)
    }
}

/// Disassembly view widget
pub struct DisassemblyView<'a> {
    /// Block for the widget
    pub block: Option<Block<'a>>,
    
    /// Disassembly lines
    pub lines: &'a [String],
    
    /// Selected line
    pub selected_line: Option<usize>,
    
    /// Scroll offset
    pub offset: usize,
    
    /// Breakpoints (line number -> enabled)
    pub breakpoints: &'a [(usize, bool)],
    
    /// Current program counter
    pub pc: Option<usize>,
}

impl<'a> DisassemblyView<'a> {
    /// Create a new disassembly view
    pub fn new(lines: &'a [String]) -> Self {
        Self {
            block: None,
            lines,
            selected_line: None,
            offset: 0,
            breakpoints: &[],
            pc: None,
        }
    }
    
    /// Set the block for the widget
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
    
    /// Set the selected line
    pub fn selected_line(mut self, selected_line: Option<usize>) -> Self {
        self.selected_line = selected_line;
        self
    }
    
    /// Set the offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }
    
    /// Set the breakpoints
    pub fn breakpoints(mut self, breakpoints: &'a [(usize, bool)]) -> Self {
        self.breakpoints = breakpoints;
        self
    }
    
    /// Set the program counter
    pub fn pc(mut self, pc: Option<usize>) -> Self {
        self.pc = pc;
        self
    }
}

impl<'a> Widget for DisassemblyView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // This is a stub implementation
        // In a real implementation, it would render the disassembly
        
        // Render the block if provided
        let area = match self.block {
            Some(block) => {
                let inner = block.inner(area);
                block.render(area, buf);
                inner
            }
            None => area,
        };
        
        // Don't render if the area is too small
        if area.width < 5 || area.height < 1 {
            return;
        }
        
        // Render as simple text for now
        let max_lines = area.height as usize;
        let end_idx = (self.offset + max_lines).min(self.lines.len());
        
        for (i, line_idx) in (self.offset..end_idx).enumerate() {
            let line = &self.lines[line_idx];
            let y = area.top() + i as u16;
            
            // Default style
            let mut style = Style::default();
            
            // Selected line style
            if let Some(selected) = self.selected_line {
                if line_idx == selected {
                    style = Style::default().fg(Color::Yellow);
                }
            }
            
            // Program counter style
            if let Some(pc) = self.pc {
                if line_idx == pc {
                    style = Style::default().fg(Color::Green);
                }
            }
            
            // Breakpoint style
            if self.breakpoints.iter().any(|(bp_idx, _)| *bp_idx == line_idx) {
                style = Style::default().fg(Color::Red);
            }
            
            buf.set_string(area.left(), y, line, style);
        }
    }
}

/// Register view widget
pub struct RegisterView<'a> {
    /// Block for the widget
    pub block: Option<Block<'a>>,
    
    /// Register names
    pub names: &'a [String],
    
    /// Register values
    pub values: &'a [String],
    
    /// Selected register
    pub selected: Option<usize>,
}

impl<'a> RegisterView<'a> {
    /// Create a new register view
    pub fn new(names: &'a [String], values: &'a [String]) -> Self {
        Self {
            block: None,
            names,
            values,
            selected: None,
        }
    }
    
    /// Set the block for the widget
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
    
    /// Set the selected register
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }
}

impl<'a> Widget for RegisterView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // This is a stub implementation
        // In a real implementation, it would render the registers
        
        // Render the block if provided
        let area = match self.block {
            Some(block) => {
                let inner = block.inner(area);
                block.render(area, buf);
                inner
            }
            None => area,
        };
        
        // Don't render if the area is too small
        if area.width < 5 || area.height < 1 {
            return;
        }
        
        // Render as simple text for now
        let max_registers = area.height as usize;
        let end_idx = max_registers.min(self.names.len()).min(self.values.len());
        
        for i in 0..end_idx {
            let name = &self.names[i];
            let value = &self.values[i];
            let y = area.top() + i as u16;
            
            // Default style
            let mut style = Style::default();
            
            // Selected register style
            if let Some(selected) = self.selected {
                if i == selected {
                    style = Style::default().fg(Color::Yellow);
                }
            }
            
            // Format as "name: value"
            let text = format!("{}: {}", name, value);
            buf.set_string(area.left(), y, text, style);
        }
    }
}

/// Stack view widget
pub struct StackView<'a> {
    /// Block for the widget
    pub block: Option<Block<'a>>,
    
    /// Stack items
    pub items: &'a [String],
    
    /// Selected item
    pub selected: Option<usize>,
    
    /// Scroll offset
    pub offset: usize,
}

impl<'a> StackView<'a> {
    /// Create a new stack view
    pub fn new(items: &'a [String]) -> Self {
        Self {
            block: None,
            items,
            selected: None,
            offset: 0,
        }
    }
    
    /// Set the block for the widget
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
    
    /// Set the selected item
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }
    
    /// Set the offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }
}

impl<'a> Widget for StackView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // This is a stub implementation
        // In a real implementation, it would render the stack
        
        // Render the block if provided
        let area = match self.block {
            Some(block) => {
                let inner = block.inner(area);
                block.render(area, buf);
                inner
            }
            None => area,
        };
        
        // Don't render if the area is too small
        if area.width < 5 || area.height < 1 {
            return;
        }
        
        // Render as simple text for now
        let max_items = area.height as usize;
        let end_idx = (self.offset + max_items).min(self.items.len());
        
        for (i, idx) in (self.offset..end_idx).enumerate() {
            let item = &self.items[idx];
            let y = area.top() + i as u16;
            
            // Default style
            let mut style = Style::default();
            
            // Selected item style
            if let Some(selected) = self.selected {
                if idx == selected {
                    style = Style::default().fg(Color::Yellow);
                }
            }
            
            buf.set_string(area.left(), y, item, style);
        }
    }
}