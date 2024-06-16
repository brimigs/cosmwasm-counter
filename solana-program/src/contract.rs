use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
};

entrypoint!(process_instruction);

#[derive(Clone, Copy, Debug, Default)]
struct Counter {
    count: u64,
}

impl Sealed for Counter {}

impl Pack for Counter {
    const LEN: usize = 8; 

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let count = u64::from_le_bytes(src.try_into().expect("Slice with incorrect length"));
        Ok(Self { count })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[..8].copy_from_slice(&self.count.to_le_bytes());
    }
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("User-Specific Counter");

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    match instruction_data[0] {
        0 => increment(account)?,
        1 => decrement(account)?,
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    Ok(())
}

fn increment(account: &AccountInfo) -> ProgramResult {
    let mut counter = Counter::unpack_from_slice(&account.try_borrow_data()?)?;
    counter.count += 1;
    Counter::pack_into_slice(&counter, &mut account.try_borrow_mut_data()?);
    Ok(())
}

fn decrement(account: &AccountInfo) -> ProgramResult {
    let mut counter = Counter::unpack_from_slice(&account.try_borrow_data()?)?;
    if counter.count > 0 {
        counter.count -= 1;
        Counter::pack_into_slice(&counter, &mut account.try_borrow_mut_data()?);
    }
    Ok(())
}