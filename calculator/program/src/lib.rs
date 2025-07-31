use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint:: ProgramResult,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CounterState { // This struct represents the state of the counter
    count: u32,
}

#[derive(BorshDeserialize, BorshSerialize)]
enum Instruction { // This enum defines the different instructions that can be executed by the program
    Init, // Initialize the counter
    Double, // Double the counter value
    Half, // Halve the counter value
    Add { amount: u32 }, // Add a specified amount to the counter
    Subtract { amount: u32 }, // Subtract a specified amount from the counter
}

entrypoint!(process_instruction); 

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(_instruction_data)?; // Deserialize the instruction data into our Instruction enum
     let mut iter=_accounts.iter(); // Create an iterator over the accounts
     let data_account = next_account_info(&mut iter)?; // Get the first account from the iterator, which is expected to be the data account
     if !data_account.is_signer{ // Check if the account is a signer
         // If the account is not a signer, return an error
         // This ensures that the account executing the instruction has the authority to modify the state
        return Err(ProgramError::MissingRequiredSignature);
     }
     let mut counter_state=CounterState::try_from_slice(&data_account.data.borrow())?; // Deserialize the state ,explanation : // we are borrowing the data from the account and then deserializing it into our CounterState struct 
     match instruction{  // Match on the instruction to determine the action
        Instruction::Init => {
            counter_state.count = 1; // Initialize the count to 1
        },
        Instruction::Double => {
            counter_state.count *= 2; // Double the count
        },
        Instruction::Half => {
            counter_state.count /= 2; // Halve the count
        },
        Instruction::Add { amount } => {
            counter_state.count += amount; // Add the specified amount to the count
        },
        Instruction::Subtract { amount } => {
            if counter_state.count < amount {
                return Err(ProgramError::InvalidInstructionData); // Prevent underflow
            }
            counter_state.count -= amount; // Subtract the specified amount from the count
        },
     }
     counter_state.serialize(&mut *data_account.data.borrow_mut())?; // Serialize the updated state back to the account data

    Ok(())
}
