// This is a workaround to allow the program to be compiled with the `unexpected_cfgs` feature enabled.
#![allow(unexpected_cfgs)]

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

// Import our instruction and processor modules
pub mod instruction;
pub mod processor;
use instruction::ProgramInstruction;
use processor::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Unpack the instruction data into our ProgramInstruction enum
    let instruction = ProgramInstruction::unpack(instruction_data)?;

    // Match on the instruction variant to handle each case
    match instruction {
        ProgramInstruction::ProcessData { data } => process_process_data(data),
    }
}
