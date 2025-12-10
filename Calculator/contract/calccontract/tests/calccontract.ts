import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ClassCal } from "../target/types/class_cal";
import { assert } from "chai";


describe("anchor-calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.calccontract as Program<ClassCal>;
  //create a new keypair 
  const newAccountKeypair = anchor.web3.Keypair.generate();

  //writing test logic 
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      account: newAccountKeypair.publicKey,
      signer: anchor.getProvider().wallet.publicKey
    }).signers([newAccountKeypair]).rpc();

    console.log("Your transaction signature", tx);
    const account = await program.account.accountShape.fetch(newAccountKeypair.publicKey);
    console.log(account.data);
    assert(account.data == 0)
  });
});
