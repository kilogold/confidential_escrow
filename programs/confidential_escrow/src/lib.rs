use anchor_lang::prelude::*;

declare_id!("8KeCuKyKT5CYeR834c9MDJZPbQ74DkqgJagdA13y58Fx");

#[program]
pub mod confidential_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
