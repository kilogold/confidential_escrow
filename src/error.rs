use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EscrowError {
    #[error("Invalid Instruction Data")]
    InvalidInstruction = 1_000_000, // Example starting offset

    // Add specific escrow errors here later
}

// Conversion from EscrowError to ProgramError
impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
} 