use std::collections::{HashMap, HashSet};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::{asm::Disassembler, lessvm::VM};

use super::shell::InputWidgetState;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum MemoryPointer {
    ProgramCounter,
}

impl MemoryPointer {
    pub fn identifier(&self) -> &'static str {
        match self {
            MemoryPointer::ProgramCounter => "pc",
        }
    }
}

#[derive(Default)]
pub struct Memory {
    pub data: Vec<u8>,
    pub access_flags: Vec<u8>,
    pub verbose: bool,
    pub follow: Option<MemoryPointer>,
}

impl Memory {
    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        state: &mut MemoryWidgetState,
        active: &mut bool,
    ) -> bool {
        if key_event.kind != KeyEventKind::Press && key_event.kind != KeyEventKind::Repeat {
            return false;
        }

        match key_event.code {
            KeyCode::Esc => {
                *active = false;
                true
            }
            KeyCode::Up => {
                if state.focus > 0 {
                    state.focus -= 1;
                }
                true
            }
            KeyCode::Down => {
                state.focus = state.focus.saturating_add(1).min(self.data.len() as u16 - 1);
                true
            }
            KeyCode::Left => {
                if state.focus >= 16 {
                    state.focus -= 16;
                } else {
                    state.focus = 0;
                }
                true
            }
            KeyCode::Right => {
                state.focus = (state.focus + 16).min(self.data.len() as u16 - 1);
                true
            }
            KeyCode::PageUp => {
                if state.focus >= 256 {
                    state.focus -= 256;
                } else {
                    state.focus = 0;
                }
                true
            }
            KeyCode::PageDown => {
                state.focus = (state.focus + 256).min(self.data.len() as u16 - 1);
                true
            }
            KeyCode::Home => {
                state.focus = 0;
                true
            }
            KeyCode::End => {
                state.focus = self.data.len() as u16 - 1;
                true
            }
            _ => false,
        }
    }
}

pub struct MemoryWidget<'a> {
    pub active: bool,
    pub memory: &'a Memory,
    pub watchpoints: &'a HashSet<super::Watchpoint>,
    pub breakpoints: &'a HashSet<usize>,
    pub vm: &'a VM,
    pub disassembler: &'a Disassembler,
}

#[derive(Default)]
pub struct MemoryWidgetState {
    _input: InputWidgetState,
    pub focus: u16,
    changed: bool,
}

impl MemoryWidgetState {
    pub fn poke(&mut self) {
        self.changed = true;
    }

    pub fn set_focus(&mut self, addr: u16) {
        self.focus = addr;
        self.changed = true;
    }
}

impl<'a> StatefulWidget for MemoryWidget<'_> {
    type State = MemoryWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.height < 3 {
            return;
        }

        let memory_len = self.memory.data.len();

        if self.memory.follow.is_some() || state.changed {
            if let Some(follow) = self.memory.follow {
                match follow {
                    MemoryPointer::ProgramCounter => {
                        state.focus = self.vm.pc as u16;
                    }
                }
            }
            state.changed = false;
        }

        let focus_addr = state.focus as usize;
        let mut start_addr = focus_addr.saturating_sub(focus_addr % 16);
        let rows_before = (area.height / 2).min(start_addr / 16) as usize;
        start_addr = start_addr.saturating_sub(rows_before * 16);
        
        let visible_rows = area.height as usize;
        let mut content = Vec::with_capacity(visible_rows);

        // Add header row
        content.push(Spans::from(vec![
            Span::raw("       "),
            Span::styled(
                " 0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F",
                Style::default().fg(Color::Yellow),
            ),
            Span::raw("  "),
            Span::styled("Disassembly", Style::default().fg(Color::Green)),
        ]));

        for row in 0..visible_rows.saturating_sub(1) {
            let row_addr = start_addr + row * 16;
            if row_addr >= memory_len {
                break;
            }

            let mut row_spans = Vec::with_capacity(18);
            row_spans.push(Span::styled(
                format!("{:04X}   ", row_addr),
                Style::default().fg(Color::DarkGray),
            ));

            // Display memory bytes
            for col in 0..16 {
                let addr = row_addr + col;
                if addr >= memory_len {
                    row_spans.push(Span::raw("   "));
                    continue;
                }

                let val = self.memory.data[addr];
                let is_pc = addr == self.vm.pc;
                let is_breakpoint = self.breakpoints.contains(&addr);
                let is_watchpoint = self.watchpoints.iter().any(|wp| {
                    if let super::Watchpoint::Address(a) = wp {
                        *a == addr
                    } else {
                        false
                    }
                });

                let style = if is_breakpoint {
                    Style::default().fg(Color::Red)
                } else if is_watchpoint {
                    Style::default().fg(Color::Blue)
                } else if is_pc {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };

                row_spans.push(Span::styled(format!("{:02X} ", val), style));
            }

            // Add disassembly if this is the start of an instruction
            if let Some(line) = self.disassembler.disassembled.iter().find(|line| line.offset == row_addr) {
                row_spans.push(Span::raw("  "));
                
                let instruction_str = format!("{:?}", line.instruction);
                let style = if row_addr == self.vm.pc {
                    Style::default().fg(Color::Green)
                } else if self.breakpoints.contains(&row_addr) {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::Reset)
                };
                
                row_spans.push(Span::styled(instruction_str, style));
            }

            content.push(Spans::from(row_spans));
        }

        Paragraph::new(content)
            .style(Style::default())
            .render(area, buf);
    }
}

impl<'a> MemoryWidget<'a> {
    pub fn write_to_file(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::prelude::*;
        
        let mut file = File::create(path)?;
        
        // Write header
        writeln!(file, "       0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F  Disassembly")?;
        writeln!(file, "---------------------------------------------------------")?;
        
        let memory_len = self.memory.data.len();
        
        for row_addr in (0..memory_len).step_by(16) {
            // Address
            write!(file, "{:04X}   ", row_addr)?;
            
            // Bytes
            for col in 0..16 {
                let addr = row_addr + col;
                if addr >= memory_len {
                    write!(file, "   ")?;
                } else {
                    let val = self.memory.data[addr];
                    write!(file, "{:02X} ", val)?;
                }
            }
            
            // Disassembly
            if let Some(line) = self.disassembler.disassembled.iter().find(|line| line.offset == row_addr) {
                write!(file, " {}", line.instruction)?;
            }
            
            writeln!(file)?;
        }
        
        Ok(())
    }
}
