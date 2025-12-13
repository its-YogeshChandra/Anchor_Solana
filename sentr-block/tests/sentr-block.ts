import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SentrBlock } from "../target/types/sentr_block";
import { expect } from "chai";

describe("sentr-block", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.sentrBlock as Program<SentrBlock>;

  //create a new keypair 
  const newKeypair = anchor.web3.Keypair.generate();

  //airdrop the account 
  await provider.connection.requestAirdrop(
    newKeypair.publicKey,
    10 * anchor.web3.LAMPORTS_PER_SOL
  )

  //create pda account 
  it("Is initialized!", async () => {
    const data = new Uint8Array([10]);

    // Add your test here.
    const tx = await program.methods.initialize(Buffer.from(data)).accounts({ signer: newKeypair.publicKey }).signers([newKeypair]).rpc();

    await provider.connection.confirmTransaction(tx);
    expect(tx).to.be.a("string");


  });
});
