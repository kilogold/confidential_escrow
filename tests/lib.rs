use {
    litesvm::LiteSVM,
    solana_instruction::{account_meta::AccountMeta, Instruction},
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_pubkey::{pubkey, Pubkey},
    solana_signer::Signer,
    solana_system_interface::instruction::transfer,
    solana_transaction::{versioned::VersionedTransaction, Transaction},
    confidential_escrow_native::instruction,
    borsh::BorshSerialize,
};

#[test]
fn test_litesvm_integration() {
    let mut svm = LiteSVM::new();

    {
        let from_keypair = Keypair::new();
        let from = from_keypair.pubkey();
        let to = Pubkey::new_unique();

        svm.airdrop(&from, 10_000).unwrap();

        let instruction = transfer(&from, &to, 64);
        let tx = Transaction::new(
            &[&from_keypair],
            Message::new(&[instruction], Some(&from)),
            svm.latest_blockhash(),
        );
        let _tx_res = svm.send_transaction(tx).unwrap();

        let from_account = svm.get_account(&from);
        let to_account = svm.get_account(&to);
        assert_eq!(from_account.unwrap().lamports, 4936);
        assert_eq!(to_account.unwrap().lamports, 64);
    }
    {
        let program_id = pubkey!("Logging111111111111111111111111111111111111");
        let account_meta = AccountMeta {
            pubkey: Pubkey::new_unique(),
            is_signer: false,
            is_writable: true,
        };
        let ix = Instruction {
            program_id,
            accounts: vec![account_meta],
            data: vec![5, 10, 11, 12, 13, 14],
        };
        let payer = Keypair::new();
        let bytes = include_bytes!("../program_bin/spl_example_logging.so");
        svm.add_program(program_id, bytes);
        svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
        let blockhash = svm.latest_blockhash();
        let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
        let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();
        // let's sim it first
        let sim_res = svm.simulate_transaction(tx.clone()).unwrap();
        let meta = svm.send_transaction(tx).unwrap();
        assert_eq!(sim_res.meta, meta);
        assert_eq!(meta.logs[1], "Program log: static string");
        assert!(meta.compute_units_consumed < 10_000); // not being precise here in case it changes

    }
}

#[test]
fn test_hello_world() {
    let mut svm = LiteSVM::new();
    let program_id = Pubkey::new_unique();
    let test_account = Keypair::new();
    let payer = Keypair::new();

    // Load and add the program to the SVM
    let bytes = include_bytes!("../target/deploy/confidential_escrow_native.so");
    svm.add_program(program_id, bytes);

    // Fund the payer account
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    // Create instruction data with discriminator and payload
    let payload = instruction::InstructionPayload {
        data: "test data".to_string(),
    };
    let mut instruction_data = vec![0u8]; // 0 is the discriminator for ProcessData
    payload.serialize(&mut instruction_data).unwrap();

    // Create the instruction
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(test_account.pubkey(), false),
            AccountMeta::new(payer.pubkey(), true),
        ],
        data: instruction_data,
    };

    // Create and send the transaction
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    // Simulate first
    let sim_res = svm.simulate_transaction(tx.clone()).unwrap();
    let meta = svm.send_transaction(tx).unwrap();

    // Verify the simulation matches the actual execution
    assert_eq!(sim_res.meta, meta);
    assert!(meta.logs.iter().any(|log| log.contains("test data")));
    println!("The output: {}", meta.pretty_logs());
}

