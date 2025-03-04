use std::{
    collections::VecDeque,
    sync::mpsc::{self, Receiver, Sender},
};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::lessvm::VM;

pub struct InputWidgetState {
    cursor_position: usize,
    input: String,
}

impl Default for InputWidgetState {
    fn default() -> Self {
        Self {
            cursor_position: 0,
            input: String::new(),
        }
    }
}

pub struct InputWidget<'a> {
    default_style: Style,
    active_style: Style,
    prompt: Span<'a>,
    active: bool,
    mask_input: bool,
    input: &'a str,
    cursor_position: usize,
}

impl<'a> InputWidget<'a> {
    fn new(
        prompt: Span<'a>,
        input: &'a str,
        cursor_position: usize,
        active: bool,
        mask_input: bool,
    ) -> Self {
        Self {
            default_style: Style::default(),
            active_style: Style::default().add_modifier(Modifier::REVERSED),
            prompt,
            active,
            mask_input,
            input,
            cursor_position,
        }
    }

    pub fn cursor_position(&self, area: Rect, state: &mut InputWidgetState) -> Option<(u16, u16)> {
        if self.active {
            let x = (state.cursor_position + self.prompt.content.width()) as u16;
            if x < area.width {
                Some((area.left() + x, area.top()))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a> StatefulWidget for InputWidget<'a> {
    type State = InputWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.height == 0 || state.input.is_empty() && !self.active {
            return;
        }

        let input_str: String = if self.mask_input {
            "*".repeat(state.input.len())
        } else {
            state.input.clone()
        };

        let text = Text::from(Spans::from(vec![
            self.prompt,
            Span::raw(input_str),
            Span::styled(" ", self.active_style),
        ]));

        Paragraph::new(text).render(area, buf);

        state.cursor_position = self.cursor_position;
    }
}

pub struct Shell {
    input_tx: Sender<String>,
    input_rx: Receiver<String>,
    output_buffer: VecDeque<Spans<'static>>,
    prompt: &'static str,
    current_input: String,
    cursor_position: usize,
    output_scroll_position: usize,
}

impl Shell {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Shell {
            input_tx: tx,
            input_rx: rx,
            output_buffer: VecDeque::with_capacity(100),
            prompt: "> ",
            current_input: String::new(),
            cursor_position: 0,
            output_scroll_position: 0,
        }
    }

    pub fn clear_output(&mut self) {
        self.output_buffer.clear();
    }

    pub fn print<T: Into<Spans<'static>>>(&mut self, content: T) {
        self.output_buffer.push_back(content.into());
        if self.output_buffer.len() > 100 {
            self.output_buffer.pop_front();
        }
    }

    pub fn echo(&mut self, input: &str) {
        self.print(format!("{}{}", self.prompt, input));
    }

    pub fn error(&mut self, message: &str) {
        self.print(Spans::from(vec![
            Span::raw("Error: "),
            Span::styled(message, Style::default().fg(Color::Red)),
        ]));
    }

    pub fn print_pc(&mut self, vm: &VM) {
        self.print(format!("PC: {:#07X}", vm.pc));
    }

    pub fn try_recv(&mut self) -> VecDeque<String> {
        let mut results = VecDeque::new();
        while let Ok(input) = self.input_rx.try_recv() {
            results.push_back(input);
        }
        results
    }

    pub fn as_output_widget(&self) -> OutputWidget {
        OutputWidget {
            content: &self.output_buffer,
            scroll_position: self.output_scroll_position,
        }
    }

    pub fn handle_input_key_event(&mut self, key_event: KeyEvent) -> bool {
        if key_event.kind != KeyEventKind::Press && key_event.kind != KeyEventKind::Repeat {
            return false;
        }

        match key_event.code {
            KeyCode::Char(c) => {
                self.current_input.insert(self.cursor_position, c);
                self.cursor_position += 1;
                true
            }
            KeyCode::Backspace => {
                if self.cursor_position > 0 {
                    self.current_input.remove(self.cursor_position - 1);
                    self.cursor_position -= 1;
                }
                true
            }
            KeyCode::Delete => {
                if self.cursor_position < self.current_input.len() {
                    self.current_input.remove(self.cursor_position);
                }
                true
            }
            KeyCode::Left => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
                true
            }
            KeyCode::Right => {
                if self.cursor_position < self.current_input.len() {
                    self.cursor_position += 1;
                }
                true
            }
            KeyCode::Home => {
                self.cursor_position = 0;
                true
            }
            KeyCode::End => {
                self.cursor_position = self.current_input.len();
                true
            }
            KeyCode::Enter => {
                let input = std::mem::take(&mut self.current_input);
                self.cursor_position = 0;
                if !input.is_empty() {
                    if let Err(e) = self.input_tx.send(input) {
                        self.error(&format!("Failed to send input: {}", e));
                    }
                }
                true
            }
            KeyCode::Esc => {
                self.current_input.clear();
                self.cursor_position = 0;
                true
            }
            KeyCode::Tab => {
                // Tab completion could be implemented here
                true
            }
            _ => false,
        }
    }

    pub fn handle_output_key_event(
        &mut self,
        key_event: KeyEvent,
        active: &mut bool,
    ) -> bool {
        if key_event.kind != KeyEventKind::Press && key_event.kind != KeyEventKind::Repeat {
            return false;
        }

        match key_event.code {
            KeyCode::Esc => {
                *active = false;
                self.output_scroll_position = 0;
                true
            }
            KeyCode::Up | KeyCode::PageUp => {
                if self.output_scroll_position + 1 < self.output_buffer.len() {
                    self.output_scroll_position += 1;
                }
                true
            }
            KeyCode::Down | KeyCode::PageDown => {
                if self.output_scroll_position > 0 {
                    self.output_scroll_position -= 1;
                }
                true
            }
            KeyCode::Home => {
                self.output_scroll_position = self.output_buffer.len().saturating_sub(1);
                true
            }
            KeyCode::End => {
                self.output_scroll_position = 0;
                true
            }
            _ => false,
        }
    }
}

impl<'a> From<&'a Shell> for InputWidget<'a> {
    fn from(shell: &'a Shell) -> Self {
        InputWidget::new(
            Span::raw(shell.prompt),
            &shell.current_input,
            shell.cursor_position,
            true,
            false,
        )
    }
}

pub struct OutputWidget<'a> {
    content: &'a VecDeque<Spans<'static>>,
    scroll_position: usize,
}

impl<'a> Widget for OutputWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }

        let max_display_lines = area.height as usize;
        let content_length = self.content.len();
        
        if content_length == 0 {
            return;
        }

        let start_idx = if self.scroll_position >= content_length {
            0
        } else {
            content_length - self.scroll_position.saturating_sub(1)
        }.saturating_sub(max_display_lines);

        let display_range = start_idx..content_length.min(start_idx + max_display_lines);
        let displayed_content = self.content.iter().skip(display_range.start).take(display_range.len());

        let text = Text::from(
            displayed_content
                .map(|spans| spans.clone())
                .collect::<Vec<_>>(),
        );

        Paragraph::new(text).render(area, buf);
    }
}
