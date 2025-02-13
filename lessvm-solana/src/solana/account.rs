use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    program::invoke_signed,
    sysvar::Sysvar,
};
use std::collections::HashMap;

pub struct AccountManager<'a> {
    accounts: &'a [AccountInfo<'a>],
    accessed: HashMap<Pubkey, bool>,
}

impl<'a> AccountManager<'a> {
    pub fn new(accounts: &'a [AccountInfo<'a>]) -> Self {
        Self {
            accounts,
            accessed: HashMap::new(),
        }
    }

    pub fn get_account(&mut self, index: usize) -> Result<&'a AccountInfo<'a>, ProgramError> {
        let account = self.accounts.get(index)
            .ok_or(ProgramError::NotEnoughAccountKeys)?;
        
        self.accessed.insert(*account.key, true);
        Ok(account)
    }

    pub fn get_account_data(&mut self, index: usize) -> Result<Vec<u8>, ProgramError> {
        let account = self.get_account(index)?;
        Ok(account.try_borrow_data()?.to_vec())
    }

    pub fn get_account_data_mut<'b>(&'b mut self, index: usize) -> Result<std::cell::RefMut<'b, [u8]>, ProgramError> {
        let account = self.get_account(index)?;
        if !account.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
        let data = account.try_borrow_mut_data()?;
        Ok(std::cell::RefMut::map(data, |d| &mut **d))
    }

    pub fn verify_signer(&mut self, index: usize) -> Result<(), ProgramError> {
        let account = self.get_account(index)?;
        if !account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }

    pub fn verify_owner(&mut self, index: usize, owner: &Pubkey) -> Result<(), ProgramError> {
        let account = self.get_account(index)?;
        if account.owner != owner {
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }

    pub fn create_account(
        &mut self,
        from_index: usize,
        to_index: usize,
        lamports: u64,
        space: usize,
        owner: &Pubkey,
        seeds: &[&[u8]],
    ) -> Result<(), ProgramError> {
        let from_account = self.get_account(from_index)?;
        let to_account = self.get_account(to_index)?;

        let rent = Rent::get()?;
        let min_lamports = rent.minimum_balance(space);

        if lamports < min_lamports {
            return Err(ProgramError::InsufficientFunds);
        }

        let ix = system_instruction::create_account(
            from_account.key,
            to_account.key,
            lamports,
            space as u64,
            owner,
        );

        invoke_signed(
            &ix,
            &[from_account.clone(), to_account.clone()],
            &[seeds],
        )?;

        Ok(())
    }

    pub fn transfer_lamports(
        &mut self,
        from_index: usize,
        to_index: usize,
        amount: u64,
    ) -> Result<(), ProgramError> {
        let from_account = self.get_account(from_index)?;
        let to_account = self.get_account(to_index)?;

        **from_account.try_borrow_mut_lamports()? = from_account
            .lamports()
            .checked_sub(amount)
            .ok_or(ProgramError::InsufficientFunds)?;

        **to_account.try_borrow_mut_lamports()? = to_account
            .lamports()
            .checked_add(amount)
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(())
    }

    pub fn verify_rent_exempt(&mut self, index: usize) -> Result<(), ProgramError> {
        let account = self.get_account(index)?;
        let rent = Rent::get()?;
        
        if !rent.is_exempt(account.lamports(), account.data_len()) {
            return Err(ProgramError::AccountNotRentExempt);
        }
        Ok(())
    }

    pub fn accounts(&self) -> &[AccountInfo<'a>] {
        self.accounts
    }

    pub fn get_accessed_accounts(&self) -> &HashMap<Pubkey, bool> {
        &self.accessed
    }

    pub fn clear_access_flags(&mut self) {
        self.accessed.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;

    fn create_test_account(
        key: Pubkey,
        lamports: u64,
        space: usize,
        owner: Pubkey,
        is_signer: bool,
        is_writable: bool,
    ) -> AccountInfo<'static> {
        let mut lamports = lamports;
        let mut data = vec![0; space];
        
        AccountInfo::new(
            &key,
            is_signer,
            is_writable,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        )
    }

    #[test]
    fn test_account_access() {
        let program_id = Pubkey::new_unique();
        let key1 = Pubkey::new_unique();
        let key2 = Pubkey::new_unique();

        let account1 = create_test_account(key1, 1000000, 32, program_id, true, true);
        let account2 = create_test_account(key2, 0, 32, program_id, false, true);

        let accounts = vec![account1, account2];
        let mut manager = AccountManager::new(&accounts);

        assert!(manager.get_account(0).is_ok());
        assert!(manager.get_account(1).is_ok());
        assert!(manager.get_account(2).is_err());

        assert!(manager.verify_signer(0).is_ok());
        assert!(manager.verify_signer(1).is_err());

        assert!(manager.verify_owner(0, &program_id).is_ok());
        assert!(manager.verify_owner(0, &Pubkey::new_unique()).is_err());

        assert_eq!(manager.get_accessed_accounts().len(), 2);
    }

    #[test]
    fn test_transfer_lamports() {
        let program_id = Pubkey::new_unique();
        let key1 = Pubkey::new_unique();
        let key2 = Pubkey::new_unique();

        let account1 = create_test_account(key1, 1000000, 32, program_id, true, true);
        let account2 = create_test_account(key2, 0, 32, program_id, false, true);

        let accounts = vec![account1, account2];
        let mut manager = AccountManager::new(&accounts);

        assert!(manager.transfer_lamports(0, 1, 500000).is_ok());
        assert_eq!(accounts[0].lamports(), 500000);
        assert_eq!(accounts[1].lamports(), 500000);

        // Test insufficient funds
        assert!(manager.transfer_lamports(0, 1, 1000000).is_err());
    }
}