//! TUI module for lessVM IDE
//!
//! This module provides a terminal-based user interface for lessVM development.

pub mod app;
pub mod event;
pub mod ui;
pub mod vscode_ui;
pub mod vscode;
pub mod syntax;
pub mod git;
pub mod widgets;

// Re-export frequently used types
pub use app::{App, AppResult};

use std::{
    io,
    path::PathBuf,
    time::{Duration, Instant},
};
use anyhow::{Context, Result};
use crossterm::{
    event::{self as crossterm_event, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use self::{
    event::handle_key_event
};

/// Run the TUI application
pub fn run() -> Result<()> {
    // Setup terminal
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;
    
    // Create app and run event loop
    let mut app = App::new();
    app.init(None)?;

    // Initialize VS Code UI components
    if app.use_vscode_ui {
        app.init_vscode_ui();
    }
    
    // Main loop
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    
    while app.running {
        // Render UI
        terminal.draw(|f| {
            if app.use_vscode_ui {
                vscode_ui::render(&mut app, f);
            } else {
                ui::render(&mut app, f);
            }
        })?;
        
        // Handle events
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        
        if crossterm_event::poll(timeout)? {
            match crossterm_event::read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                        // Ctrl+C handling
                        app.running = false;
                    } else {
                        // Handle other key events
                        handle_key_event(&mut app, key)?;
                    }
                },
                Event::Resize(_, _) => {
                    // Terminal resized, force redraw
                    terminal.draw(|f| {
                        if app.use_vscode_ui {
                            vscode_ui::render(&mut app, f);
                        } else {
                            ui::render(&mut app, f);
                        }
                    })?;
                },
                _ => {},
            }
        }
        
        // Tick logic
        if last_tick.elapsed() >= tick_rate {
            app.tick()?;
            last_tick = Instant::now();
        }
    }
    
    // Restore terminal
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    ).context("Failed to leave alternate screen")?;
    
    Ok(())
}

/// Run the TUI application with a specified file
pub fn run_with_file(path: PathBuf) -> Result<()> {
    // Setup terminal
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;
    
    // Create app and run event loop
    let mut app = App::new();
    app.init(Some(path))?;

    // Initialize VS Code UI components
    if app.use_vscode_ui {
        app.init_vscode_ui();
    }
    
    // Main loop
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    
    while app.running {
        // Render UI
        terminal.draw(|f| {
            if app.use_vscode_ui {
                vscode_ui::render(&mut app, f);
            } else {
                ui::render(&mut app, f);
            }
        })?;
        
        // Handle events
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        
        if crossterm_event::poll(timeout)? {
            match crossterm_event::read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                        // Ctrl+C handling
                        app.running = false;
                    } else {
                        // Handle other key events
                        handle_key_event(&mut app, key)?;
                    }
                },
                Event::Resize(_, _) => {
                    // Terminal resized, force redraw
                    terminal.draw(|f| {
                        if app.use_vscode_ui {
                            vscode_ui::render(&mut app, f);
                        } else {
                            ui::render(&mut app, f);
                        }
                    })?;
                },
                _ => {},
            }
        }
        
        // Tick logic
        if last_tick.elapsed() >= tick_rate {
            app.tick()?;
            last_tick = Instant::now();
        }
    }
    
    // Restore terminal
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    ).context("Failed to leave alternate screen")?;
    
    Ok(())
}