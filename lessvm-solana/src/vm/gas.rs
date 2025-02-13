use super::VMError;
use smallvec::SmallVec;

#[repr(C, align(64))]
pub struct Gas {
    remaining: u64,
    last_checkpoint: u64,
    checkpoints: SmallVec<[u64; 16]>,
}

impl Gas {
    pub fn new(limit: u64) -> Self {
        Self {
            remaining: limit,
            last_checkpoint: limit,
            checkpoints: SmallVec::new(),
        }
    }

    #[inline(always)]
    pub fn consume(&mut self, amount: u64) -> Result<(), VMError> {
        self.remaining = self.remaining.checked_sub(amount)
            .ok_or(VMError::OutOfGas)?;
        Ok(())
    }

    #[inline(always)]
    pub fn checkpoint(&mut self) {
        self.checkpoints.push(self.remaining);
        self.last_checkpoint = self.remaining;
    }

    #[inline(always)]
    pub fn revert_to_checkpoint(&mut self) -> Result<(), VMError> {
        self.remaining = self.checkpoints.pop()
            .ok_or(VMError::OutOfGas)?;
        self.last_checkpoint = self.remaining;
        Ok(())
    }

    #[inline(always)]
    pub fn commit_checkpoint(&mut self) {
        self.last_checkpoint = self.remaining;
        let _ = self.checkpoints.pop();
    }

    #[inline(always)]
    pub fn remaining(&self) -> u64 {
        self.remaining
    }

    #[inline(always)]
    pub fn used(&self) -> u64 {
        self.last_checkpoint - self.remaining
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.remaining = self.last_checkpoint;
        self.checkpoints.clear();
    }

    // Memory expansion gas cost
    pub fn memory_expansion_cost(&self, current_size: usize, new_size: usize) -> u64 {
        if new_size <= current_size {
            return 0;
        }

        // Similar to EVM memory expansion cost
        let current_words = (current_size + 31) / 32;
        let new_words = (new_size + 31) / 32;
        
        if new_words <= current_words {
            return 0;
        }

        // Cost formula: new_words * 3 + new_words * new_words / 512
        let base_cost = (new_words as u64) * 3;
        let quadratic_cost = ((new_words * new_words) as u64) / 512;
        
        // Subtract the cost we already paid for current size
        let current_cost = (current_words as u64) * 3 + ((current_words * current_words) as u64) / 512;
        base_cost + quadratic_cost - current_cost
    }

    // Account access cost
    pub fn account_access_cost(&self, is_cold: bool) -> u64 {
        if is_cold {
            2600 // Cold account access
        } else {
            100 // Warm account access
        }
    }

    // SLOAD cost
    pub fn storage_load_cost(&self, is_cold: bool) -> u64 {
        if is_cold {
            2100 // Cold storage load
        } else {
            100 // Warm storage load
        }
    }

    // SSTORE cost based on value changes
    pub fn storage_store_cost(&self, current_value: u64, new_value: u64) -> u64 {
        if current_value == new_value {
            100 // No change
        } else if current_value == 0 {
            20000 // New storage slot
        } else if new_value == 0 {
            5000 // Storage slot cleared
        } else {
            5000 // Storage slot modified
        }
    }
} 