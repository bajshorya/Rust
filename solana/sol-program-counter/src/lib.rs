use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Counter {
    pub count: u32,
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        InstructionType::Increment(value) => {
            msg!("increased!!!!");
            counter_data.count += value;
        }
        InstructionType::Decrement(value) => {
            msg!("decreased!!!!");
            counter_data.count -= value;
        }
    }
    counter_data.serialize(&mut *acc.data.borrow_mut());
    msg!("success")
    Ok(())
}
//87Hnqgbc6AoraKh8ykbMLMPb8XfmGH6MRgjooEkGFgdm