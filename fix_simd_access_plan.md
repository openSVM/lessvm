# Plan to Fix SIMD Access in VM Implementation

## Current Issue
The VM's vector_add implementation is trying to directly access the private `data` field of the Stack struct to perform SIMD operations. This violates encapsulation and won't compile.

## Solution Plan

### 1. Add SIMD Support Methods to Stack

Add two new methods to the Stack struct in `stack.rs`:

```rust
impl Stack {
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
        Ok(self.data.as_ptr().add(self.depth() - 8))
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
        Ok(self.data.as_mut_ptr().add(self.depth() - 8))
    }
}
```

### 2. Update Vector Add Implementation

Modify the vector_add function in `core.rs` to use the new safe methods:

```rust
#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn vector_add(&mut self) -> Result<(), VMError> {
    let values = _mm256_loadu_si256(self.stack.as_simd_ptr()? as *const __m256i);
    let result = _mm256_add_epi64(values, values);
    _mm256_storeu_si256(self.stack.as_simd_mut_ptr()? as *mut __m256i, result);
    Ok(())
}
```

## Benefits

1. **Maintains Encapsulation**: The Stack's data remains private
2. **Safe Interface**: New methods include proper bounds checking
3. **Performance**: Preserves SIMD optimization capabilities
4. **Clean API**: Provides clear methods specifically for SIMD operations

## Implementation Steps

1. Add new SIMD support methods to Stack
2. Update vector_add implementation
3. Add tests to verify SIMD operations work correctly
4. Document safety requirements for SIMD methods

## Testing Plan

1. Test stack underflow conditions
2. Verify SIMD operations produce correct results
3. Ensure stack integrity is maintained after operations
4. Test with various stack sizes and values