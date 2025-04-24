use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

// Define the instruction payload for our program
#[derive(BorshSerialize, BorshDeserialize)]
pub struct InstructionPayload {
    pub data: String,
}

// Define the instruction variants our program supports
pub enum ProgramInstruction {
    // Example instruction that takes a string payload
    ProcessData { data: String },
}

impl ProgramInstruction {
    // Unpack the instruction data into our instruction enum
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // First byte is the instruction discriminator
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // Match on the discriminator to determine which instruction to unpack
        match variant {
            0 => {
                let payload = InstructionPayload::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::ProcessData { data: payload.data })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
} 