use crate::error::EscrowError; // Assuming custom errors will be defined here
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token_2022::{
    extension::ExtensionType,
    instruction as token_2022_instruction,
    state::Account,
    ID as TOKEN_2022_PROGRAM_ID,
};
use solana_program::program_pack::Pack;

pub struct Processor;
impl Processor {
    /// Processes the InitializeEscrow instruction
    ///
    /// Accounts expected:
    /// 0. `[signer]` Signer account (payer for PDA creation)
    /// 1. `[]` Mint account (Token-2022)
    /// 2. `[writable]` Escrow token account PDA (to be created)
    /// 3. `[]` System program
    /// 4. `[]` Token-2022 program
    /// 5. `[]` Rent sysvar
    pub fn process_initialize_escrow(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        msg!("Processing InitializeEscrow instruction");

        let account_info_iter = &mut accounts.iter();

        // Get accounts
        let signer_account = next_account_info(account_info_iter)?;
        let mint_account = next_account_info(account_info_iter)?;
        let escrow_token_pda = next_account_info(account_info_iter)?;
        let system_program_account = next_account_info(account_info_iter)?;
        let token_program_account = next_account_info(account_info_iter)?;
        let rent_sysvar_account = next_account_info(account_info_iter)?;

        // --- Validation ---
        // Check signer
        if !signer_account.is_signer {
            msg!("Error: Signer account must sign the transaction");
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Check programs
        if *system_program_account.key != solana_program::system_program::id() {
            msg!("Error: Invalid system program account");
            return Err(ProgramError::IncorrectProgramId);
        }
        if *token_program_account.key != TOKEN_2022_PROGRAM_ID {
             msg!("Error: Invalid token program account. Expected Token-2022.");
             return Err(ProgramError::IncorrectProgramId);
        }
        // Check rent sysvar
        if *rent_sysvar_account.key != solana_program::sysvar::rent::id() {
             msg!("Error: Invalid rent sysvar account");
             return Err(ProgramError::IncorrectProgramId);
        }

        // --- PDA Derivation & Creation ---
        msg!("Deriving PDA address...");
        let escrow_seeds = &[
            b"escrow".as_ref(),
            mint_account.key.as_ref(),
            signer_account.key.as_ref(),
        ];
        let (pda_address, bump_seed) = Pubkey::find_program_address(escrow_seeds, program_id);

        // Verify the derived PDA matches the account passed in
        if pda_address != *escrow_token_pda.key {
            msg!("Error: Escrow token account PDA address mismatch. Expected {}, got {}", pda_address, escrow_token_pda.key);
            return Err(ProgramError::InvalidSeeds);
        }

        msg!("PDA address: {}, Bump: {}", pda_address, bump_seed);
        let escrow_signer_seeds = &[
            b"escrow".as_ref(),
            mint_account.key.as_ref(),
            signer_account.key.as_ref(),
            &[bump_seed],
        ];
        let escrow_signer_seeds = &[&escrow_signer_seeds[..]]; //Rebind as a slice.

        // Calculate space and rent
        let base_space = Account::LEN;
        let rent = Rent::get()?;
        let base_lamports = rent.minimum_balance(base_space);
        msg!("Required base space: {}, Rent lamports: {}", base_space, base_lamports);

        // Create the PDA account via CPI to System Program
        msg!("Creating PDA account...");
        invoke(
            &system_instruction::create_account(
                signer_account.key,       // Payer
                escrow_token_pda.key,     // Account to create
                base_lamports,            // Lamports
                base_space as u64,         // Space
                token_program_account.key, // Owner (Token-2022 program)
            ),
            &[
                signer_account.clone(), // Must be mutable if signer pays
                escrow_token_pda.clone(),
                system_program_account.clone(),
            ],
        )?;
        msg!("PDA account created.");

        // --- Initialize PDA as Token Account ---
        msg!("Initializing PDA as Token-2022 account...");
        let initialize_ix = token_2022_instruction::initialize_account3(
            token_program_account.key, // Token program ID
            escrow_token_pda.key,      // Account to initialize
            mint_account.key,          // Mint
            &pda_address,              // Authority (the PDA itself)
        )?;

        invoke_signed(
            &initialize_ix,
            &[
                token_program_account.clone(),
                escrow_token_pda.clone(),
                mint_account.clone(),
            ],
            escrow_signer_seeds, // PDA signs this CPI
        )?;

        msg!("Escrow token account PDA initialized successfully.");

        // --- Step 3: Reallocate for Extension ---
        msg!("Reallocating PDA account for Confidential Transfer extension...");
        invoke_signed(
            &token_2022_instruction::reallocate(
                token_program_account.key,      // Token program ID
                escrow_token_pda.key,           // Account to reallocate
                signer_account.key,             // Payer for rent increase
                &pda_address,                   // Owner/Authority of the token account (the PDA)
                &[&pda_address],                // Signers for the owner (just the PDA itself)
                &[ExtensionType::ConfidentialTransferAccount], // Extensions to add space for
            )?,
            &[
                escrow_token_pda.clone(), // The account being reallocated (needs to be writable)
                signer_account.clone(), // Payer needs to be writable if paying rent
                system_program_account.clone(), // Needed for rent calculation by reallocate
                // The PDA is the authority, implicitly included via invoke_signed seeds
            ],
            escrow_signer_seeds, // PDA signs this CPI
        )?;

        msg!("Escrow token account PDA initialized and reallocated successfully.");
        Ok(())
    }

    /// Processes the ProcessData instruction
    pub fn process_process_data(
        data: String,
    ) -> ProgramResult {
        msg!("Processing data: {}", data);
        // Here you would implement the actual logic for processing the data
        Ok(())
    }
} 