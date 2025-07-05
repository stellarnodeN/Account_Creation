use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, read_keypair_file, Signer},
    system_instruction,
    transaction::Transaction,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to Solana devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(
        rpc_url.to_string(),
        CommitmentConfig::confirmed(),
    );

    // Load the existing turbin3-wallet keypair
    let payer = read_keypair_file("turbin3-wallet.json").expect("Failed to read keypair file");
    println!("Payer public key: {}", payer.pubkey());

    // Generate a new keypair for the new account
    let new_account = Keypair::new();
    println!("New account public key: {}", new_account.pubkey());

    // Specify space (in bytes) for the new account
    let space = 0; // No extra space

    // Calculate rent-exempt minimum lamports
    let rent = client.get_minimum_balance_for_rent_exemption(space)?;
    println!("Rent-exempt minimum: {} lamports", rent);

    // Create the create account instruction
    let create_account_instruction = system_instruction::create_account(
        &payer.pubkey(),
        &new_account.pubkey(),
        rent,
        space as u64,
        &solana_program::system_program::id(),
    );

    // Get the latest blockhash
    let recent_blockhash = client.get_latest_blockhash()?;

    // Create and sign the transaction
    let mut transaction = Transaction::new_with_payer(
        &[create_account_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &new_account], recent_blockhash);

    // Send and confirm the transaction
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction signature: {}", signature);
    println!("New account created successfully!");

    Ok(())
} 