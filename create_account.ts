// This script creates a new account on the Solana blockchain.
// It uses the Gill library to create the account and send the transaction.    

import {
  createTransaction,
  createSolanaClient,
  signTransactionMessageWithSigners,
  getMinimumBalanceForRentExemption,
  generateKeyPairSigner
} from "gill";
import { loadKeypairSignerFromFile } from "gill/node";
import {
  getCreateAccountInstruction,
  SYSTEM_PROGRAM_ADDRESS
} from "gill/programs";

const { rpc, sendAndConfirmTransaction } = createSolanaClient({
  urlOrMoniker: "devnet"
});

const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

const signer = await loadKeypairSignerFromFile();

const newAccount = await generateKeyPairSigner();

const space = 0n; // any extra space in the account

const rentLamports = getMinimumBalanceForRentExemption(space);
// const rentLamports = await rpc.getMinimumBalanceForRentExemption(space).send();

const tx = createTransaction({
  version: "legacy",
  feePayer: signer,
  instructions: [
    getCreateAccountInstruction({
      lamports: rentLamports,
      newAccount: newAccount,
      payer: signer,
      space: space,
      programAddress: SYSTEM_PROGRAM_ADDRESS
    })
  ],
  latestBlockhash
});

const signedTransaction = await signTransactionMessageWithSigners(tx);
const signature = await sendAndConfirmTransaction(signedTransaction);
console.log("Transaction Signature:", signature);