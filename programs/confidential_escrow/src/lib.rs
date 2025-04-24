use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};
declare_id!("8KeCuKyKT5CYeR834c9MDJZPbQ74DkqgJagdA13y58Fx");

#[program]
pub mod confidential_escrow {
    use super::*;

    pub fn initialize_config(
        ctx: Context<Initialize>,
        expected_taker_amount: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.maker_mint = ctx.accounts.maker_mint.key();
        config.taker_mint = ctx.accounts.taker_mint.key();
        config.expected_taker_amount = expected_taker_amount;
        Ok(())
    }
}

#[account]
pub struct EscrowConfig {
    pub maker_mint: Pubkey,
    pub taker_mint: Pubkey,
    pub expected_taker_amount: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 8, // 8 for discriminator, 32 for each pubkey, 8 for u64
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, EscrowConfig>,
    
    /// The mint account for the maker's token (Token2022)
    pub maker_mint: InterfaceAccount<'info, Mint>,
    
    /// The mint account for the taker's token (Token2022)
    pub taker_mint: InterfaceAccount<'info, Mint>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
