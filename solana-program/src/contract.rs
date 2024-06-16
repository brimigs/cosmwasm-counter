use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
};

entrypoint!(process_instruction);

#[derive(Clone, Copy, Debug, Default)]
struct Counter {
    count: u64,
}

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("User-Specific Counter");

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter = Counter::unpack(&account.data.borrow())?;

    match instruction_data[0] {
        0 => counter.count += 1,
        1 => counter.count -= 1,
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    counter.pack(&mut account.data.borrow_mut())?;

    Ok(())
}