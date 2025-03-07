use std::{
    io,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::LevelFilter;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};
// use tui_logger::TuiLoggerWidget;

use crate::lessvm::{VM, VMState};

/// Render state
struct RenderState {
    /// Key event from input
    key_event: Option<KeyCode>,
    
    /// Whether to request a new frame render
    render_requested: bool,
    
    /// Whether the application is running
    running: bool,
    
    /// Current tab index
    tab_index: usize,
    
    /// Terminal size
    terminal_size: Rect,
}

/// Render controller for communicating with the render thread
pub struct RenderController {
    /// Shared render state
    state: Arc<Mutex<RenderState>>,
}

impl RenderController {
    /// Create a new render controller
    fn new() -> Self {
        RenderController {
            state: Arc::new(Mutex::new(RenderState {
                key_event: None,
                render_requested: true,
                running: true,
                tab_index: 0,
                terminal_size: Rect::new(0, 0, 0, 0),
            })),
        }
    }
    
    /// Get key event if available
    pub fn take_key_event(&mut self) -> Option<KeyCode> {
        let mut state = self.state.lock().unwrap();
        let key = state.key_event;
        state.key_event = None;
        key
    }
    
    /// Request a new frame render
    pub fn request_render(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.render_requested = true;
    }
    
    /// Check if render is requested
    fn is_render_requested(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.render_requested
    }
    
    /// Clear render request
    fn clear_render_request(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.render_requested = false;
    }
    
    /// Set key event
    fn set_key_event(&mut self, key: KeyCode) {
        let mut state = self.state.lock().unwrap();
        state.key_event = Some(key);
    }
    
    /// Check if application is running
    pub fn is_running(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.running
    }
    
    /// Set running state
    pub fn set_running(&mut self, running: bool) {
        let mut state = self.state.lock().unwrap();
        state.running = running;
    }
    
    /// Get current tab index
    fn tab_index(&self) -> usize {
        let state = self.state.lock().unwrap();
        state.tab_index
    }
    
    /// Set current tab index
    fn set_tab_index(&mut self, index: usize) {
        let mut state = self.state.lock().unwrap();
        state.tab_index = index;
    }
    
    /// Set terminal size
    fn set_terminal_size(&mut self, size: Rect) {
        let mut state = self.state.lock().unwrap();
        state.terminal_size = size;
    }
    
    /// Get terminal size
    fn terminal_size(&self) -> Rect {
        let state = self.state.lock().unwrap();
        state.terminal_size
    }
}

/// Spawn a render thread
pub fn spawn_render_thread(
    vm: &VM,
    with_logging: bool,
) -> (Arc<Mutex<RenderController>>, JoinHandle<Result<()>>) {
    let controller = Arc::new(Mutex::new(RenderController::new()));
    let controller_clone = controller.clone();
    
    // Clone VM fields for the rendering thread
    let vm_render = VMRender::from_vm(vm);
    
    let handle = thread::spawn(move || {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        
        // Create app state
        let mut app = App {
            controller: controller_clone,
            vm: vm_render,
            fps: 0.0,
            frame_time: 0.0,
            show_logs: with_logging,
        };
        
        let res = render_loop(&mut terminal, &mut app);
        
        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        
        res
    });
    
    (controller, handle)
}

/// Rendering loop
fn render_loop<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let mut last_frame = Instant::now();
    
    while app.controller.lock().unwrap().is_running() {
        // Handle events
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) = event::read()? {
                let mut controller = app.controller.lock().unwrap();
                
                match code {
                    KeyCode::Char('q') => {
                        controller.set_running(false);
                    },
                    KeyCode::Tab => {
                        let next_tab = (controller.tab_index() + 1) % 3;
                        controller.set_tab_index(next_tab);
                        controller.request_render();
                    },
                    KeyCode::BackTab => {
                        let next_tab = (controller.tab_index() + 2) % 3;
                        controller.set_tab_index(next_tab);
                        controller.request_render();
                    },
                    _ => {
                        controller.set_key_event(code);
                        controller.request_render();
                    }
                }
            }
        }
        
        // Update FPS stats
        let now = Instant::now();
        let frame_time = now.duration_since(last_frame);
        app.frame_time = frame_time.as_secs_f64() * 1000.0; // ms
        app.fps = 1.0 / frame_time.as_secs_f64();
        last_frame = now;
        
        // Render if requested
        let render_requested = app.controller.lock().unwrap().is_render_requested();
        if render_requested {
            terminal.draw(|f| {
                let size = f.size();
                app.controller.lock().unwrap().set_terminal_size(size);
                
                // Clear render request
                app.controller.lock().unwrap().clear_render_request();
                
                // Render UI
                ui(f, app);
            })?;
        }
    }
    
    Ok(())
}

/// Rendering UI
fn ui(f: &mut Frame, app: &mut App) {
    let size = f.size();
    
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Tab bar
            Constraint::Min(5),    // Content
            Constraint::Length(1), // Status bar
        ])
        .split(size);
    
    // Render tab bar
    let tab_titles = vec!["VM", "Memory", "Stack"];
    let tabs = Tabs::new(
        tab_titles.iter().map(|t| Line::from(Span::raw(*t))).collect()
    )
    .block(Block::default().borders(Borders::ALL).title("LessVM Debugger"))
    .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    .select(app.controller.lock().unwrap().tab_index());
    
    f.render_widget(tabs, chunks[0]);
    
    // Render content based on selected tab
    let tab_index = app.controller.lock().unwrap().tab_index();
    match tab_index {
        0 => render_vm_tab(f, &app.vm, chunks[1]),
        1 => render_memory_tab(f, &app.vm, chunks[1]),
        2 => render_stack_tab(f, &app.vm, chunks[1]),
        _ => {}
    }
    
    // Render status bar
    let status = format!(
        "Status: {} | PC: {:#06X} | FPS: {:.1} | Frame time: {:.2}ms | Press 'q' to quit",
        app.vm.state, app.vm.pc, app.fps, app.frame_time
    );
    let status_bar = Paragraph::new(status)
        .style(Style::default().fg(Color::White));
    
    f.render_widget(status_bar, chunks[2]);
    
    // Render logs if enabled
    if app.show_logs {
        let logs_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(chunks[1])[1];
        
        // Create a simple log placeholder instead of TuiLoggerWidget
        let logs = Paragraph::new("Logs are disabled in this version")
            .block(Block::default().title("Logs").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
            
        f.render_widget(logs, logs_area);
    }
}

/// Render VM tab
fn render_vm_tab(f: &mut Frame, vm: &VMRender, area: Rect) {
    let vm_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);
    
    // Render registers
    let mut registers = Vec::new();
    for (i, reg) in vm.registers.iter().enumerate() {
        registers.push(format!("R{}: {:#010X}", i, reg));
    }
    
    let registers_items: Vec<ListItem> = registers
        .iter()
        .map(|r| ListItem::new(Line::from(Span::raw(r))))
        .collect();
    
    let registers_list = List::new(registers_items)
        .block(Block::default().title("Registers").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_widget(registers_list, vm_layout[0]);
    
    // Render VM state
    let state = format!(
        "State: {}\nPC: {:#06X}\nCycles: {}\nGas used: {}\nGas limit: {}",
        vm.state, vm.pc, vm.cycle_counter, vm.gas_used, vm.gas_limit
    );
    
    let state_widget = Paragraph::new(state)
        .block(Block::default().title("VM State").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(state_widget, vm_layout[1]);
}

/// Render memory tab
fn render_memory_tab(f: &mut Frame, vm: &VMRender, area: Rect) {
    // Calculate visible memory range
    let pc = vm.pc;
    let start_addr = pc.saturating_sub(64);
    let end_addr = (pc + 64).min(vm.memory.len());
    
    // Format memory blocks
    let mut memory_lines = Vec::new();
    for addr in (start_addr..end_addr).step_by(16) {
        let mut line = format!("{:#06X}: ", addr);
        
        // Add hex representation
        for i in 0..16 {
            if addr + i < end_addr {
                line.push_str(&format!("{:02X} ", vm.memory[addr + i]));
            } else {
                line.push_str("   ");
            }
        }
        
        line.push_str("  ");
        
        // Add ASCII representation
        for i in 0..16 {
            if addr + i < end_addr {
                let byte = vm.memory[addr + i];
                if byte >= 32 && byte <= 126 {
                    line.push(byte as char);
                } else {
                    line.push('.');
                }
            } else {
                line.push(' ');
            }
        }
        
        memory_lines.push(line);
    }
    
    // Highlight current PC
    let pc_index = (pc - start_addr) / 16;
    
    let memory_items: Vec<ListItem> = memory_lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            if i == pc_index {
                ListItem::new(Line::from(Span::styled(
                    line,
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                )))
            } else {
                ListItem::new(Line::from(Span::raw(line)))
            }
        })
        .collect();
    
    let memory_list = List::new(memory_items)
        .block(Block::default().title("Memory").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(memory_list, area);
}

/// Render stack tab
fn render_stack_tab(f: &mut Frame, vm: &VMRender, area: Rect) {
    let mut stack_items = Vec::new();
    
    // Display stack from top to bottom
    for (i, value) in vm.stack.iter().rev().enumerate() {
        stack_items.push(format!("{}: {:#010X}", vm.stack.len() - i - 1, value));
    }
    
    let stack_list_items: Vec<ListItem> = stack_items
        .iter()
        .map(|s| ListItem::new(Line::from(Span::raw(s))))
        .collect();
    
    let stack_list = List::new(stack_list_items)
        .block(Block::default().title("Stack").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(stack_list, area);
}

/// VM Render State - simplified version of VM for rendering
#[derive(Clone)]
struct VMRender {
    pub pc: usize,
    pub memory: Vec<u8>,
    pub registers: [u32; 16],
    pub stack: Vec<u32>,
    pub state: String,
    pub cycle_counter: u32,
    pub gas_used: u64,
    pub gas_limit: u64,
}

impl VMRender {
    /// Create a VM render state from a VM
    fn from_vm(vm: &VM) -> Self {
        VMRender {
            pc: vm.pc,
            memory: vm.memory.clone(),
            registers: vm.registers,
            stack: vm.stack.clone(),
            state: format!("{}", vm.state),
            cycle_counter: vm.cycle_counter,
            gas_used: vm.gas_used(),
            gas_limit: vm.gas_limit(),
        }
    }
    
    /// Update from VM
    fn update_from_vm(&mut self, vm: &VM) {
        self.pc = vm.pc;
        self.memory = vm.memory.clone();
        self.registers = vm.registers;
        self.stack = vm.stack.clone();
        self.state = format!("{}", vm.state);
        self.cycle_counter = vm.cycle_counter;
        self.gas_used = vm.gas_used();
        self.gas_limit = vm.gas_limit();
    }
}

/// Application state
struct App {
    /// Render controller
    controller: Arc<Mutex<RenderController>>,
    
    /// VM render state
    vm: VMRender,
    
    /// FPS counter
    fps: f64,
    
    /// Frame time in milliseconds
    frame_time: f64,
    
    /// Whether to show logs
    show_logs: bool,
}
