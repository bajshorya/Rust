import { expect, test } from "bun:test";
import * as borsh from "borsh";

import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { COUNTER_SIZE, schema } from "./types";
const PROGRAM_ID = new PublicKey(
  "87Hnqgbc6AoraKh8ykbMLMPb8XfmGH6MRgjooEkGFgdm"
);
let adminAccount = Keypair.generate();
let dataAccount = Keypair.generate();
test(
  "Account is initialized and funded",
  async () => {
    const connection = new Connection("http://127.0.0.1:8899", "confirmed");
    const airdropSignature = await connection.requestAirdrop(
      adminAccount.publicKey,
      1 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(
      {
        signature: airdropSignature,
        blockhash: (await connection.getLatestBlockhash("confirmed")).blockhash,
        lastValidBlockHeight: (
          await connection.getLatestBlockhash("confirmed")
        ).lastValidBlockHeight,
      },
      "confirmed"
    );
    const balance = await connection.getBalance(
      adminAccount.publicKey,
      "confirmed"
    );
    const data = await connection.getAccountInfo(
      adminAccount.publicKey,
      "confirmed"
    );
    console.log(`Balance: ${balance / LAMPORTS_PER_SOL} SOL`);
    console.log(data);
    expect(balance).toBe(LAMPORTS_PER_SOL);
    //airdrop done
    const lamports = await connection.getMinimumBalanceForRentExemption(
      COUNTER_SIZE
    );
    const ix = SystemProgram.createAccount({
      fromPubkey: adminAccount.publicKey,
      lamports,
      space: COUNTER_SIZE,
      programId: PROGRAM_ID,
      newAccountPubkey: dataAccount.publicKey,
    });
    const createAccountTxn = new Transaction();
    createAccountTxn.add(ix);
    const signature = await connection.sendTransaction(createAccountTxn, [
      adminAccount,
      dataAccount,
    ]);
    await connection.confirmTransaction(signature);
    console.log("dataAccountPublicKey", dataAccount.publicKey.toBase58());
    const dataAccountInfo = await connection.getAccountInfo(
      dataAccount.publicKey
    );
    const counter = borsh.deserialize(schema, dataAccountInfo?.data);
    console.log(counter.count);
    expect(counter.count).toBe(0);
  },
  { timeout: 60000 }
);
