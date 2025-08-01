import { describe, test, expect, beforeAll } from "bun:test";
import { LiteSVM } from "litesvm";
import {
  PublicKey,
  Transaction,
  SystemProgram,
  Keypair,
  LAMPORTS_PER_SOL,
  TransactionInstruction,
} from "@solana/web3.js";
import * as path from "path";
describe("Counter Calculator Program Tests", () => {
  let svm: LiteSVM;
  let programId: PublicKey; // PublicKey for the calculator program
  // This should be the actual program ID of your deployed calculator program
  let dataAccount: Keypair; // Keypair for the data account
  // This account will hold the state of the calculator
  let userAccount: Keypair; // Keypair for the user account
  // This account will be used to send transactions and pay fees

  const programPath = path.join(import.meta.dir, "calculator.so");
  beforeAll(() => {
    // Load the program before running tests
    svm = new LiteSVM();
    programId = PublicKey.unique(); // Replace with actual program ID
    svm.addProgramFromFile(programId, programPath); // Load the program
    dataAccount = new Keypair(); // Create a new keypair for the data account
    userAccount = new Keypair(); // Create a new keypair for the user account
    svm.airdrop(userAccount.publicKey, BigInt(LAMPORTS_PER_SOL)); // Airdrop SOL to the user account
    let ix = SystemProgram.createAccount({
      fromPubkey: userAccount.publicKey,
      newAccountPubkey: dataAccount.publicKey,
      lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))),
      space: 4,
      programId: programId,
    });
    let tx = new Transaction().add(ix); // Create a transaction to initialize the data account
    tx.recentBlockhash = svm.latestBlockhash(); // Get the latest blockhash
    tx.sign(userAccount, dataAccount); // Sign the transaction with both user and data accounts
    svm.sendTransaction(tx); // Send the transaction to the blockchain
    svm.expireBlockhash(); // Expire the blockhash to ensure the transaction is processed
  });
  test("init", () => {
    const instruction = new TransactionInstruction({
      programId, // Program ID of the calculator program
      keys: [
        {
          pubkey: dataAccount.publicKey, // Data account to be initialized
          isSigner: true, // Signer is the data account itself
          isWritable: true, // Writable because we will write to it
        },
      ],
      data: Buffer.from([0]), // 0 for init operation
    });
    const transaction = new Transaction().add(instruction); // Create a new transaction with the instruction
    transaction.recentBlockhash = svm.latestBlockhash(); // Get the latest blockhash
    transaction.feePayer = userAccount.publicKey; // Set the fee payer to the user account
    transaction.sign(dataAccount, userAccount); // Sign the transaction with both accounts
    let signature = svm.sendTransaction(transaction); // Send the transaction to the blockchain

    const updatedAccountData = svm.getAccount(dataAccount.publicKey); // Get the updated account data
    if (!updatedAccountData) {
      throw new Error("Data account not found after initialization");
    }
    expect(updatedAccountData.data[0]).toBe(1); // Check if the first byte is 1, indicating initialization
    expect(updatedAccountData.data[1]).toBe(0);
    expect(updatedAccountData.data[2]).toBe(0);
    expect(updatedAccountData.data[3]).toBe(0);
    // Check if the account is initialized correctly i.e the first byte is 1 and the remaining bytes are 0
    // so the number initialized is 1
  });
  test("double", () => {
    const instruction = new TransactionInstruction({
      programId, // Program ID of the calculator program
      keys: [
        {
          pubkey: dataAccount.publicKey, // Data account to be initialized
          isSigner: true, // Signer is the data account itself
          isWritable: true, // Writable because we will write to it
        },
      ],
      data: Buffer.from([1]), // 0 for double operation
    });
    const transaction = new Transaction().add(instruction); // Create a new transaction with the instruction
    transaction.recentBlockhash = svm.latestBlockhash(); // Get the latest blockhash
    transaction.feePayer = userAccount.publicKey; // Set the fee payer to the user account
    transaction.sign(dataAccount, userAccount); // Sign the transaction with both accounts
    let signature = svm.sendTransaction(transaction); // Send the transaction to the blockchain

    const updatedAccountData = svm.getAccount(dataAccount.publicKey); // Get the updated account data
    if (!updatedAccountData) {
      throw new Error("Data account not found after initialization");
    }
    expect(updatedAccountData.data[0]).toBe(2); // Check if the first byte is 2, indicating double of 1
    expect(updatedAccountData.data[1]).toBe(0);
    expect(updatedAccountData.data[2]).toBe(0);
    expect(updatedAccountData.data[3]).toBe(0);
  });
  test("half", () => {
    const instruction = new TransactionInstruction({
      programId, // Program ID of the calculator program
      keys: [
        {
          pubkey: dataAccount.publicKey, // Data account to be initialized
          isSigner: true, // Signer is the data account itself
          isWritable: true, // Writable because we will write to it
        },
      ],
      data: Buffer.from([2]), // 2 for half operation
    });
    const transaction = new Transaction().add(instruction); // Create a new transaction with the instruction
    transaction.recentBlockhash = svm.latestBlockhash(); // Get the latest blockhash
    transaction.feePayer = userAccount.publicKey; // Set the fee payer to the user account
    transaction.sign(dataAccount, userAccount); // Sign the transaction with both accounts
    let signature = svm.sendTransaction(transaction); // Send the transaction to the blockchain

    const updatedAccountData = svm.getAccount(dataAccount.publicKey); // Get the updated account data
    if (!updatedAccountData) {
      throw new Error("Data account not found after initialization");
    }
    expect(updatedAccountData.data[0]).toBe(1); // Check if the first byte is 1, indicating half of 2
    expect(updatedAccountData.data[1]).toBe(0);
    expect(updatedAccountData.data[2]).toBe(0);
    expect(updatedAccountData.data[3]).toBe(0);
  });
  test("Add", () => {
    const instruction = new TransactionInstruction({
      programId, // Program ID of the calculator program
      keys: [
        {
          pubkey: dataAccount.publicKey, // Data account to be initialized
          isSigner: true, // Signer is the data account itself
          isWritable: true, // Writable because we will write to it
        },
      ],
      data: Buffer.from([3, 5, 0, 0, 0]), // 3 for add operation, and 5 as 4-byte little-endian u32 which is 5
    });
    const transaction = new Transaction().add(instruction); // Create a new transaction with the instruction
    transaction.recentBlockhash = svm.latestBlockhash(); // Get the latest blockhash
    transaction.feePayer = userAccount.publicKey; // Set the fee payer to the user account
    transaction.sign(dataAccount, userAccount); // Sign the transaction with both accounts
    let signature = svm.sendTransaction(transaction); // Send the transaction to the blockchain

    const updatedAccountData = svm.getAccount(dataAccount.publicKey); // Get the updated account data
    if (!updatedAccountData) {
      throw new Error("Data account not found after initialization");
    }
    expect(updatedAccountData.data[0]).toBe(6); // Check if the first byte is 6, indicating 1 + 5
    expect(updatedAccountData.data[1]).toBe(0);
    expect(updatedAccountData.data[2]).toBe(0);
    expect(updatedAccountData.data[3]).toBe(0);
  });
  test("subtract", () => {
    const instruction = new TransactionInstruction({
      programId, // Program ID of the calculator program
      keys: [
        {
          pubkey: dataAccount.publicKey,
          isSigner: true,
          isWritable: true,
        },
      ],
      data: Buffer.from([4, 2, 0, 0, 0]), // 4 for subtract operation, and 2 as 4-byte little-endian u32 which is 2
    });
    const transaction = new Transaction().add(instruction);
    transaction.recentBlockhash = svm.latestBlockhash();
    transaction.feePayer = userAccount.publicKey;
    transaction.sign(dataAccount, userAccount);
    svm.sendTransaction(transaction);

    const updatedAccountData = svm.getAccount(dataAccount.publicKey);
    if (!updatedAccountData) {
      throw new Error("Data account not found after subtract");
    }
    expect(updatedAccountData.data[0]).toBe(4); // Check if the first byte is 4, indicating 6 - 2
    expect(updatedAccountData.data[1]).toBe(0);
    expect(updatedAccountData.data[2]).toBe(0);
    expect(updatedAccountData.data[3]).toBe(0);
  });
});
