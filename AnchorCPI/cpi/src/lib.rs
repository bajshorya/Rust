use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};
#[derive(BorshSerialize, BorshDeserialize)]
struct CounterState {
    count: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
enum CounterInstruction {
    Initialize,
    Double,
    Half,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    account: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CounterInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    match instruction {
        CounterInstruction::Initialize => {
            msg!("Initializing counter state");
            
        }
    }
}
