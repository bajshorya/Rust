import { expect, test } from "bun:test";
import { LiteSVM } from "litesvm";
import {
  PublicKey,
  Transaction,
  SystemProgram,
  Keypair,
  LAMPORTS_PER_SOL,
  TransactionInstruction,
} from "@solana/web3.js";
test("CPI WORKS WELL", async () => {
  const svm = new LiteSVM();
  const doubleContract = PublicKey.unique();
  const cpiContract = PublicKey.unique();
  svm.addProgramFromFile(doubleContract, "./double.so");
  svm.addProgramFromFile(cpiContract, "./cpi.so");
  const userAcc = new Keypair();
  const dataAcc = new Keypair();

  svm.airdrop(userAcc.publicKey, BigInt(BigInt(1000_000_000)));
  createDataAccountOnChain(svm, dataAcc, userAcc, doubleContract);
  function doubleIt() {
    let ix = new TransactionInstruction({
      keys: [
        { pubkey: dataAcc.publicKey, isSigner: true, isWritable: true },
        {
          pubkey: doubleContract,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: cpiContract,
      data: Buffer.from(""),
    });
    let blockhash = svm.latestBlockhash();
    const transaction = new Transaction().add(ix);
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = userAcc.publicKey;
    transaction.sign(userAcc, dataAcc);
    const res = svm.sendTransaction(transaction);
    // console.log(res);
    svm.expireBlockhash();
  }
  doubleIt();
  doubleIt();

  doubleIt();

  doubleIt();

  const dataAccount = svm.getAccount(dataAcc.publicKey);
  expect(dataAccount?.data[0]).toBe(8);
  expect(dataAccount?.data[1]).toBe(0);
  expect(dataAccount?.data[2]).toBe(0);
  expect(dataAccount?.data[3]).toBe(0);
});
function createDataAccountOnChain(
  svm: LiteSVM,
  dataAccount: Keypair,
  payer: Keypair,
  contractPubkey: PublicKey
) {
  const blockhash = svm.latestBlockhash();
  //   console.log(payer.publicKey.toBase58());
  const ixs = [
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: dataAccount.publicKey,
      lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))),
      space: 4,
      programId: contractPubkey,
    }),
  ];
  const tx = new Transaction();
  tx.recentBlockhash = blockhash;
  tx.feePayer = payer.publicKey;
  tx.add(...ixs);
  tx.sign(payer, dataAccount);
  svm.sendTransaction(tx);
}
