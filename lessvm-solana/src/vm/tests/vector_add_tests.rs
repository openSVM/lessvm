use super::super::*;
use solana_program::clock::Epoch;
use solana_program::pubkey::Pubkey;
use solana_program::account_info::AccountInfo;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn create_test_account(lamports: u64) -> (Pubkey, Vec<u8>, AccountInfo<'static>) {
    let key = Pubkey::new_unique();
    let mut lamports = lamports;
    let mut data = vec![0; 32];
    
    AccountInfo::new(
        &key,
        true,
        true,
        &mut lamports,
        &mut data,
        &Pubkey::new_unique(),
        false,
        Epoch::default(),
    )
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_vector_add() {
    let program_id = Pubkey::new_unique();
    let (_, _, account) = create_test_account(1000000);
    let accounts = vec![account];
    
    let mut vm = VM::new(&program_id, &accounts, &[]);
    
    // Push 8 values to the stack: [1, 2, 3, 4, 5, 6, 7, 8]
    for i in 1..=8 {
        vm.stack.push(Value(i)).unwrap();
    }
    
    // Execute vector_add operation
    unsafe {
        vm.vector_add().unwrap();
    }
    
    // The stack should now have 4 values: [6, 8, 10, 12]
    // These are the sums of [1+5, 2+6, 3+7, 4+8]
    assert_eq!(vm.stack.depth(), 4);
    assert_eq!(vm.stack.pop().unwrap().0, 12);
    assert_eq!(vm.stack.pop().unwrap().0, 10);
    assert_eq!(vm.stack.pop().unwrap().0, 8);
    assert_eq!(vm.stack.pop().unwrap().0, 6);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_vector_add_with_different_values() {
    let program_id = Pubkey::new_unique();
    let (_, _, account) = create_test_account(1000000);
    let accounts = vec![account];
    
    let mut vm = VM::new(&program_id, &accounts, &[]);
    
    // Push 8 values to the stack with different patterns
    vm.stack.push(Value(10)).unwrap();
    vm.stack.push(Value(20)).unwrap();
    vm.stack.push(Value(30)).unwrap();
    vm.stack.push(Value(40)).unwrap();
    vm.stack.push(Value(5)).unwrap();
    vm.stack.push(Value(15)).unwrap();
    vm.stack.push(Value(25)).unwrap();
    vm.stack.push(Value(35)).unwrap();
    
    // Execute vector_add operation
    unsafe {
        vm.vector_add().unwrap();
    }
    
    // The stack should now have 4 values: [15, 35, 55, 75]
    // These are the sums of [10+5, 20+15, 30+25, 40+35]
    assert_eq!(vm.stack.depth(), 4);
    assert_eq!(vm.stack.pop().unwrap().0, 75);
    assert_eq!(vm.stack.pop().unwrap().0, 55);
    assert_eq!(vm.stack.pop().unwrap().0, 35);
    assert_eq!(vm.stack.pop().unwrap().0, 15);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_vector_add_stack_underflow() {
    let program_id = Pubkey::new_unique();
    let (_, _, account) = create_test_account(1000000);
    let accounts = vec![account];
    
    let mut vm = VM::new(&program_id, &accounts, &[]);
    
    // Push only 7 values to the stack (not enough for vector_add)
    for i in 1..=7 {
        vm.stack.push(Value(i)).unwrap();
    }
    
    // Execute vector_add operation - should fail with StackUnderflow
    unsafe {
        let result = vm.vector_add();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), VMError::StackUnderflow);
    }
}