// This is a workaround to allow the program to be compiled with the `unexpected_cfgs` feature enabled.
#![allow(unexpected_cfgs)]

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// Import our instruction and processor modules
pub mod instruction;
pub mod processor;
pub mod state; // Assuming state definitions might be needed later
pub mod error; // Assuming custom errors might be needed

use instruction::ProgramInstruction;
use processor::Processor;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,      // Program ID of this program
    accounts: &[AccountInfo], // Accounts passed into the instruction
    instruction_data: &[u8],  // Instruction data
) -> ProgramResult {
    msg!("Entrypoint");
    // Unpack the instruction data into our ProgramInstruction enum
    let instruction = ProgramInstruction::unpack(instruction_data)?;

    // Call the processor based on the instruction variant
    match instruction {
        ProgramInstruction::InitializeEscrow => {
            msg!("Instruction: InitializeEscrow");
            Processor::process_initialize_escrow(program_id, accounts)
        }
        ProgramInstruction::ProcessData { data } => {
             msg!("Instruction: ProcessData");
             // Assuming process_process_data is now part of the Processor struct
             Processor::process_process_data(data)
        }
    }
}
