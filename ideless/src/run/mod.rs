use std::{
    sync::{Arc, atomic::{AtomicBool, Ordering}, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use anyhow::{Result, anyhow};
use crossterm::event::KeyCode;
use log::{debug, error, info};

use crate::{
    lessvm::{VM, VMState},
    dbg::Debugger,
    render::RenderController,
};

/// Runner for the VM emulation
pub struct Runner {
    /// VM reference
    lessvm: VM,
    
    /// Debugger (optional)
    debugger: Option<Debugger>,
    
    /// Cycles per frame
    cycles_per_frame: u32,
    
    /// Running state
    running: Arc<AtomicBool>,
    
    /// Paused state
    paused: Arc<AtomicBool>,
    
    /// FPS counter and limiter
    fps_counter: FpsCounter,
}

impl Runner {
    /// Create a new VM runner for LessVM
    pub fn new_lessvm(lessvm: VM, debugger: Option<Debugger>) -> Self {
        let cpf = debugger.as_ref().map_or(100, |d| d.cycles_per_frame());
        
        Runner {
            lessvm,
            debugger,
            cycles_per_frame: cpf,
            running: Arc::new(AtomicBool::new(true)),
            paused: Arc::new(AtomicBool::new(false)),
            fps_counter: FpsCounter::new(),
        }
    }
    
    /// Get reference to the VM
    pub fn lessvm(&self) -> &VM {
        &self.lessvm
    }
    
    /// Get mutable reference to the VM
    pub fn lessvm_mut(&mut self) -> &mut VM {
        &mut self.lessvm
    }
    
    /// Get reference to the debugger
    pub fn debugger(&self) -> Option<&Debugger> {
        self.debugger.as_ref()
    }
    
    /// Get mutable reference to the debugger
    pub fn debugger_mut(&mut self) -> Option<&mut Debugger> {
        self.debugger.as_mut()
    }
    
    /// Get cycles per frame
    pub fn cycles_per_frame(&self) -> u32 {
        self.cycles_per_frame
    }
    
    /// Set cycles per frame
    pub fn set_cycles_per_frame(&mut self, cpf: u32) {
        self.cycles_per_frame = cpf;
        if let Some(debugger) = &mut self.debugger {
            debugger.set_cycles_per_frame(cpf);
        }
    }
    
    /// Check if the runner is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
    
    /// Set the running state
    pub fn set_running(&self, running: bool) {
        self.running.store(running, Ordering::Relaxed);
    }
    
    /// Check if the runner is paused
    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::Relaxed)
    }
    
    /// Set the paused state
    pub fn set_paused(&self, paused: bool) {
        self.paused.store(paused, Ordering::Relaxed);
    }
    
    /// Toggle the paused state
    pub fn toggle_pause(&self) {
        self.paused.fetch_xor(true, Ordering::Relaxed);
    }
    
    /// Run a single frame
    pub fn run_frame(&mut self) -> Result<()> {
        // In debug mode, delegate to the debugger
        if let Some(debugger) = &mut self.debugger {
            return debugger.run_frame(&mut self.lessvm);
        }
        
        // Otherwise, run cycles directly on the VM
        match self.lessvm.state {
            VMState::Ready | VMState::Running => {
                self.lessvm.run(self.cycles_per_frame)?;
                Ok(())
            }
            VMState::Halted => {
                debug!("VM halted at PC={:#06X}", self.lessvm.pc);
                Ok(())
            }
            VMState::Error(ref msg) => {
                error!("VM error: {}", msg);
                Ok(())
            }
        }
    }
    
    /// Handle keyboard input
    pub fn handle_key(&mut self, key: KeyCode) -> Result<()> {
        // In debug mode, delegate to the debugger
        if let Some(debugger) = &mut self.debugger {
            return debugger.handle_key(&mut self.lessvm, key);
        }
        
        // Otherwise, handle keys directly
        match key {
            KeyCode::Char('q') => {
                self.set_running(false);
            }
            KeyCode::Char(' ') => {
                self.toggle_pause();
            }
            KeyCode::Char('r') => {
                self.lessvm.reset();
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Get FPS statistics
    pub fn fps_stats(&self) -> (f64, f64) {
        self.fps_counter.stats()
    }
    
    /// Update FPS counter
    pub fn update_fps(&mut self) {
        self.fps_counter.update();
    }
}

/// Run thread function
pub fn spawn_run_thread(
    mut runner: Runner,
    render_controller: Arc<Mutex<RenderController>>,
    debug_mode: bool,
    with_logging: bool,
) -> JoinHandle<Result<()>> {
    // Clone atomic flags for the thread
    let running = runner.running.clone();
    let paused = runner.paused.clone();
    
    thread::spawn(move || {
        // Welcome message
        if with_logging {
            info!("LessVM Runner started");
            info!("Press 'q' to quit, 'space' to pause/resume, 'r' to reset");
            if debug_mode {
                info!("Debug mode enabled");
                info!("Press 'n' for next instruction, 'c' to continue, 'b' to set breakpoint");
            }
        }
        
        // Main loop
        while running.load(Ordering::Relaxed) {
            let frame_start = Instant::now();
            
            // Handle events from render thread
            {
                let mut controller = render_controller.lock().unwrap();
                if let Some(key) = controller.take_key_event() {
                    if let Err(err) = runner.handle_key(key) {
                        error!("Error handling key: {}", err);
                    }
                }
            }
            
            // Run a frame if not paused
            if !paused.load(Ordering::Relaxed) {
                if let Err(err) = runner.run_frame() {
                    error!("Error running frame: {}", err);
                }
            }
            
            // Update the render controller
            {
                let mut controller = render_controller.lock().unwrap();
                controller.request_render();
            }
            
            // Update FPS counter
            runner.update_fps();
            
            // Frame timing
            let frame_time = frame_start.elapsed();
            let target_frame_time = Duration::from_secs_f64(1.0 / 60.0);
            
            if frame_time < target_frame_time {
                thread::sleep(target_frame_time - frame_time);
            }
        }
        
        Ok(())
    })
}

/// FPS counter and limiter
struct FpsCounter {
    frame_count: u32,
    fps: f64,
    last_update: Instant,
    frame_times: Vec<Duration>,
}

impl FpsCounter {
    fn new() -> Self {
        FpsCounter {
            frame_count: 0,
            fps: 0.0,
            last_update: Instant::now(),
            frame_times: Vec::with_capacity(100),
        }
    }
    
    fn update(&mut self) {
        self.frame_count += 1;
        
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_update);
        self.last_update = now;
        
        // Track frame times
        self.frame_times.push(frame_time);
        if self.frame_times.len() > 100 {
            self.frame_times.remove(0);
        }
        
        // Update FPS every second
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        if elapsed >= 1.0 {
            self.fps = self.frame_count as f64 / elapsed;
            self.frame_count = 0;
        }
    }
    
    fn stats(&self) -> (f64, f64) {
        // Calculate average frame time
        let avg_frame_time = if self.frame_times.is_empty() {
            0.0
        } else {
            let sum: Duration = self.frame_times.iter().sum();
            sum.as_secs_f64() / self.frame_times.len() as f64
        };
        
        (self.fps, avg_frame_time)
    }
}
