use super::VMError;

#[repr(C, align(64))]
pub struct Memory {
    data: [u8; 1024],
    size: usize,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0; 1024],
            size: 0,
        }
    }

    #[inline(always)]
    pub fn get_byte(&self, offset: usize) -> Option<u8> {
        self.data.get(offset).copied()
    }

    #[inline(always)]
    fn bounds_check(&self, offset: usize, size: usize) -> Result<(), VMError> {
        offset.checked_add(size)
            .filter(|&end| end <= self.data.len())
            .ok_or(VMError::InvalidMemoryAccess)?;
        Ok(())
    } 

    #[inline(always)]
    pub fn store(&mut self, offset: usize, value: &[u8]) -> Result<(), VMError> {
        self.bounds_check(offset, value.len())?;
        self.data[offset..offset + value.len()].copy_from_slice(value);
        self.size = self.size.max(offset + value.len());
        Ok(())
    }

    #[inline(always)]
    pub fn store8(&mut self, offset: usize, value: u8) -> Result<(), VMError> {
        self.bounds_check(offset, 1)?;
        self.data[offset] = value;
        self.size = self.size.max(offset + 1);
        Ok(())
    }

    #[inline(always)]
    pub fn load(&self, offset: usize, len: usize) -> Result<&[u8], VMError> {
        self.bounds_check(offset, len)?;
        Ok(&self.data[offset..offset + len])
    }

    #[inline(always)]
    pub fn load8(&self, offset: usize) -> Result<u8, VMError> {
        self.bounds_check(offset, 1)?;
        Ok(self.data[offset])
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.data.fill(0);
        self.size = 0;
    }

    #[inline(always)]
    pub fn copy(&mut self, dest: usize, src: usize, len: usize) -> Result<(), VMError> {
        self.bounds_check(src, len)?;
        self.bounds_check(dest, len)?;
        self.data.copy_within(src..src + len, dest);
        self.size = self.size.max(dest + len);
        Ok(())
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.size]
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    // Gas cost calculation for memory operations
    pub fn expansion_cost(&self, offset: usize, size: usize) -> u64 {
        let new_size = offset.saturating_add(size);
        if new_size <= self.size {
            return 0;
        }

        // Cost formula similar to EVM:
        // words = (new_size + 31) / 32
        // cost = words * 3 + words * words / 512
        let words = (new_size + 31) / 32;
        ((words as u64) * 3) + ((words * words) as u64) / 512
    }
} 