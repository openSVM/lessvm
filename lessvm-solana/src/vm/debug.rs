use super::{OpCode, Stack, Memory, Value};
use std::fmt;

pub trait Tracer {
    fn trace_execution(&mut self, trace: ExecutionTrace);
}

pub struct DefaultTracer;

impl Tracer for DefaultTracer {
    fn trace_execution(&mut self, _trace: ExecutionTrace) {
        // No-op implementation
    }
}

#[derive(Debug)]
pub struct ExecutionTrace {
    pub pc: usize,
    pub opcode: OpCode,
    pub gas_used: u64,
    pub gas_remaining: u64,
    pub stack_depth: usize,
    pub memory_size: usize,
}

pub struct StackView<'a>(&'a Stack);
pub struct MemoryView<'a>(&'a Memory);

impl<'a> fmt::Debug for StackView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack [")?;
        for i in 0..self.0.depth() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", self.0.get_value(i).unwrap_or(Value(0)).0)?;
        }
        write!(f, "]")
    }
}

impl<'a> fmt::Debug for MemoryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.0.size();
        if size == 0 {
            return write!(f, "Memory []");
        }

        writeln!(f, "Memory [")?;
        let mut offset = 0;
        write!(f, "{:08x}:", offset)?;
        for i in 0..size {
            if i > 0 && i % 16 == 0 {
                writeln!(f)?;
                offset = i;
                write!(f, "{:08x}:", offset)?;
            }
            if i % 2 == 0 { write!(f, " ")?; }
            write!(f, "{:02x}", self.0.get_byte(offset + i).unwrap_or(0))?;
        }
        write!(f, "]")
    }
}