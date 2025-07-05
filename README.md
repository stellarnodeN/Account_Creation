# Creating an account on Solana

A TypeScript repo to create new accounts on Solana using the gill library.

## What it does

This script demonstrates how to create a new Solana account using the System Program's `createAccount` instruction. It:

- Connects to Solana devnet using gill
- Uses an existing wallet as the payer
- Creates a new account with rent-exempt minimum balance
- Returns the transaction signature and new account public key

## Usage

```bash
npm start
```

Or run directly:
```bash
ts-node --esm create_account.ts
```

## Requirements

- Node.js
- TypeScript and ts-node

## Based on

This implementation uses the [gill library](https://gill.site/docs) and is based on the [official Solana Cookbook example](https://solana.com/hi/developers/cookbook/accounts/create-account) for creating accounts on Solana.

## Project Structure

```
├── package.json            # Node.js dependencies
├── tsconfig.json           # TypeScript configuration
├── create_account.ts       # Main account creation logic
├── .gitignore              # Git ignore rules
└── README.md               # This file
``` 