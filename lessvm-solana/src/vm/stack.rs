use bytemuck::{Pod, Zeroable};
use super::VMError;
use smallvec::{SmallVec, smallvec};

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Value(pub u64);

impl Value {
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn from_u64(value: u64) -> Self {
        Self(value)
    }

    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }

    pub fn checked_mul(&self, other: &Self) -> Option<Self> {
        self.0.checked_mul(other.0).map(Self)
    }

    pub fn checked_div(&self, other: &Self) -> Option<Self> {
        if other.0 == 0 {
            None
        } else {
            self.0.checked_div(other.0).map(Self)
        }
    }
}

unsafe impl Pod for Value {}
unsafe impl Zeroable for Value {}

impl Default for Value {
    fn default() -> Self {
        Self(0)
    }
}


#[repr(C, align(64))]
pub struct Stack {
    data: SmallVec<[Value; 64]>,
    frames: SmallVec<[usize; 8]>,
    top: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: smallvec![Value(0); 64],
            frames: SmallVec::new(),
            top: 0,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, value: Value) -> Result<(), VMError> {
        if self.top >= self.data.len() {
            return Err(VMError::StackOverflow);
        }
        self.data.push(value);
        self.top += 1;
        Ok(())
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Result<Value, VMError> {
        if self.top == 0 {
            return Err(VMError::StackUnderflow);
        }
        self.top -= 1;
        Ok(self.data.pop().unwrap())
    }

    #[inline(always)]
    pub fn peek(&self) -> Result<Value, VMError> {
        if self.top == 0 {
            return Err(VMError::StackUnderflow);
        }
        Ok(self.data[self.top - 1])
    }

    #[inline(always)]
    pub fn dup(&mut self, n: usize) -> Result<(), VMError> {
        if n >= self.top {
            return Err(VMError::StackUnderflow);
        }
        let value = self.data[self.top - 1 - n];
        self.push(value)
    }

    #[inline(always)]
    pub fn swap(&mut self, n: usize) -> Result<(), VMError> {
        if n >= self.top {
            return Err(VMError::StackUnderflow);
        }
        let idx1 = self.top - 1;
        let idx2 = self.top - 1 - n;
        self.data.swap(idx1, idx2);
        Ok(())
    }

    #[inline(always)]
    pub fn depth(&self) -> usize {
        self.top
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.data.clear();
        self.top = 0;
        self.frames.clear();
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.top >= self.data.len()
    }

    #[inline(always)]
    pub fn get_value(&self, index: usize) -> Option<Value> {
        self.data.get(index).copied()
    }

    #[inline(always)]
    pub fn push_frame(&mut self) -> Result<(), VMError> {
        if self.frames.len() >= 8 {
            return Err(VMError::StackOverflow);
        }
        self.frames.push(self.top);
        Ok(())
    }

    #[inline(always)]
    pub fn pop_frame(&mut self) -> Result<(), VMError> {
        let frame_start = self.frames.pop()
            .ok_or(VMError::StackUnderflow)?;
        self.data.truncate(frame_start);
        self.top = frame_start;
        Ok(())
    }

    /// Returns a pointer to the last 8 elements for SIMD reading
    ///
    /// # Safety
    /// Caller must ensure:
    /// - At least 8 elements are on the stack
    /// - Pointer is only used for reading
    #[inline(always)]
    pub fn as_simd_ptr(&self) -> Result<*const Value, VMError> {
        if self.depth() < 8 {
            return Err(VMError::StackUnderflow);
        }
        unsafe {
            Ok(self.data.as_ptr().add(self.depth() - 8))
        }
    }

    /// Returns a mutable pointer to the last 8 elements for SIMD writing
    ///
    /// # Safety
    /// Caller must ensure:
    /// - At least 8 elements are on the stack
    /// - Pointer is only used for writing valid Values
    #[inline(always)]
    pub fn as_simd_mut_ptr(&mut self) -> Result<*mut Value, VMError> {
        if self.depth() < 8 {
            return Err(VMError::StackUnderflow);
        }
        unsafe {
            Ok(self.data.as_mut_ptr().add(self.depth() - 8))
        }
    }
}
