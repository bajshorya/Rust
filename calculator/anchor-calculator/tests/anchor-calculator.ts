import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorCalculator } from "../target/types/anchor_calculator";
import { assert } from "chai";

describe("anchor-calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const newAccount = anchor.web3.Keypair.generate();

  const program = anchor.workspace
    .anchorCalculator as Program<AnchorCalculator>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .init(10)
      .accounts({
        signer: anchor.getProvider().wallet.publicKey,
        account: newAccount.publicKey,
      })
      .signers([newAccount])
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert(account.num == 10);
  });
  it("Is double!", async () => {
    // Add your test here.
    const tx = await program.methods
      .double()
      .accounts({
        signer: anchor.getProvider().wallet.publicKey,
        account: newAccount.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert(account.num == 20);
  });
  it("Add!", async () => {
    // Add your test here.
    const tx = await program.methods
      .add(5)
      .accounts({
        signer: anchor.getProvider().wallet.publicKey,
        account: newAccount.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert(account.num == 25);
  });
  it("Half!", async () => {
    // Add your test here.
    const tx = await program.methods
      .half()
      .accounts({
        signer: anchor.getProvider().wallet.publicKey,
        account: newAccount.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert(account.num == 12);
    console.log(account.num);
  });
});
