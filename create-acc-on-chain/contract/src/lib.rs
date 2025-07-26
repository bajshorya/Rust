use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction::create_account,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let pda = next_account_info(iter)?;
    let user_account = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;
    let seeds = &[user_account.key.as_ref(), b"user"];

    let (pda_public_key, bump) = Pubkey::find_program_address(seeds, &program_id);

    let ix = create_account(user_account.key, pda.key, 1000000000, 8, program_id);
    let signer_seeds = &[user_account.key.as_ref(), b"user", &[bump]];
    invoke_signed(&ix, accounts, &[signer_seeds]);
    Ok(())
}
