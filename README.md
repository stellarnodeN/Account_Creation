# Creating an account on Solana

A simple Rust script to create new accounts on Solana.

## What it does

This script demonstrates how to create a new Solana account using the System Program's `createAccount` instruction. It:

- Connects to Solana devnet
- Uses an existing wallet as the payer
- Creates a new account with rent-exempt minimum balance
- Returns the transaction signature and new account public key

## Usage

```bash
cargo run
```

## Requirements

- Rust toolchain
- Solana wallet file (`turbin3-wallet.json`)
- Internet connection for devnet access

## Based on

This implementation is based on the [official Solana Cookbook example](https://solana.com/hi/developers/cookbook/accounts/create-account) for creating accounts on Solana.

## Project Structure

```
├── Cargo.toml          # Rust dependencies
├── src/main.rs         # Main account creation logic
├── .gitignore          # Git ignore rules
└── README.md           # This file
``` 