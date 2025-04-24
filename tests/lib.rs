use {
    litesvm::LiteSVM,
    solana_instruction::{account_meta::AccountMeta, Instruction},
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_pubkey::{pubkey, Pubkey},
    solana_signer::Signer,
    solana_system_interface::instruction::transfer,
    solana_transaction::{versioned::VersionedTransaction, Transaction},
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
        println!("{}", meta.pretty_logs());
    }
}

#[test]
fn test_hello_world() {
    let mut svm = LiteSVM::new();

    // Define the program ID
    let program_id = Pubkey::new_unique();
    
    // Create a payer account
    let payer = Keypair::new();
    
    // Load and add the program to the SVM
    let bytes = include_bytes!("../target/deploy/confidential_escrow_native.so");
    svm.add_program(program_id, bytes);
    
    // Fund the payer account
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    
    // Create a simple instruction (no accounts needed for this basic program)
    let ix = Instruction {
        program_id,
        accounts: vec![],
        data: vec![],
    };
    
    // Create and send the transaction
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();
    
    // Simulate first
    let sim_res = svm.simulate_transaction(tx.clone()).unwrap();
    let meta = svm.send_transaction(tx).unwrap();
    
    // Verify the simulation matches the actual execution
    assert_eq!(sim_res.meta, meta);
    
    // Check that the program logged "Hello, world!"
    assert_eq!(meta.logs[1], "Program log: Hello, world!");
}

