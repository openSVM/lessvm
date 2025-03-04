use std::collections::VecDeque;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::lessvm::VM;

#[derive(Default)]
pub struct HistoryEntry {
    pub registers: [u32; 16],
    pub pc: usize,
    pub stack: Vec<u32>,
    pub memory: Vec<u8>,
}

impl HistoryEntry {
    pub fn from_vm(vm: &VM) -> Self {
        HistoryEntry {
            registers: vm.registers,
            pc: vm.pc,
            stack: vm.stack.clone(),
            memory: vm.memory.clone(),
        }
    }

    pub fn apply_to_vm(&self, vm: &mut VM) {
        vm.registers.copy_from_slice(&self.registers);
        vm.pc = self.pc;
        vm.stack = self.stack.clone();
        vm.memory = self.memory.clone();
        vm.state = crate::lessvm::VMState::Ready;
    }
}

pub struct History {
    undo_stack: VecDeque<HistoryEntry>,
    redo_stack: VecDeque<HistoryEntry>,
    max_history: usize,
}

impl Default for History {
    fn default() -> Self {
        History {
            undo_stack: VecDeque::with_capacity(100),
            redo_stack: VecDeque::with_capacity(100),
            max_history: 100,
        }
    }
}

impl History {
    pub fn push(&mut self, entry: HistoryEntry) {
        self.undo_stack.push_back(entry);
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.pop_front();
        }
    }

    pub fn clear_redo_history(&mut self) {
        self.redo_stack.clear();
    }

    pub fn redo_amount(&self) -> usize {
        self.redo_stack.len()
    }

    pub fn undo(&mut self, vm: &mut VM, amount: usize, memory_access_flags: &mut [u8]) -> usize {
        let mut undone = 0;

        for _ in 0..amount {
            if let Some(entry) = self.undo_stack.pop_back() {
                self.redo_stack.push_back(HistoryEntry::from_vm(vm));
                entry.apply_to_vm(vm);
                undone += 1;
            } else {
                break;
            }
        }

        undone
    }

    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        active: &mut bool,
        seek_payload: &mut (usize, bool),
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
                *seek_payload = (1, false);
                true
            }
            KeyCode::Down => {
                *seek_payload = (1, true);
                true
            }
            KeyCode::PageUp => {
                *seek_payload = (10, false);
                true
            }
            KeyCode::PageDown => {
                *seek_payload = (10, true);
                true
            }
            _ => false,
        }
    }
}

pub struct HistoryWidget<'a> {
    pub history: &'a History,
    pub active: bool,
    pub border: tui::widgets::Borders,
}

impl<'a> Widget for HistoryWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        let block = Block::default()
            .title(" History ")
            .title_style(if self.active {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            })
            .borders(self.border);

        let inner_area = block.inner(area);
        block.render(area, buf);

        if inner_area.height < 2 {
            return;
        }

        let undo_count = self.history.undo_stack.len();
        let redo_count = self.history.redo_stack.len();

        let mut content = Vec::new();

        content.push(Spans::from(vec![
            Span::raw("Undo stack: "),
            Span::styled(
                format!("{}", undo_count),
                Style::default().fg(if undo_count > 0 {
                    Color::Green
                } else {
                    Color::DarkGray
                }),
            ),
            Span::raw(" entries"),
        ]));

        content.push(Spans::from(vec![
            Span::raw("Redo stack: "),
            Span::styled(
                format!("{}", redo_count),
                Style::default().fg(if redo_count > 0 {
                    Color::Yellow
                } else {
                    Color::DarkGray
                }),
            ),
            Span::raw(" entries"),
        ]));

        if self.active {
            content.push(Spans::from(""));
            content.push(Spans::from(vec![
                Span::styled("Up", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - Undo one step"),
            ]));
            content.push(Spans::from(vec![
                Span::styled("Down", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - Redo one step"),
            ]));
            content.push(Spans::from(vec![
                Span::styled("PageUp", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - Undo 10 steps"),
            ]));
            content.push(Spans::from(vec![
                Span::styled("PageDown", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - Redo 10 steps"),
            ]));
            content.push(Spans::from(vec![
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - Exit history mode"),
            ]));
        }

        Paragraph::new(content).render(inner_area, buf);
    }
}
