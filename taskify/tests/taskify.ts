import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Taskify } from "../target/types/taskify";
import { assert } from "chai";

describe("taskify", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.taskify as Program<Taskify>;
  const mainKeyPair = anchor.web3.Keypair.generate();

  //airdrop the solana 
  await provider.connection.requestAirdrop(mainKeyPair.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);

  //make the seed Prefix 
  const seedprefix = Buffer.from("hello_word");

  const derivePda = () => {
    //must match you rust seeds exactly 
    return anchor.web3.PublicKey.findProgramAddressSync(
      [seedprefix, mainKeyPair.publicKey.toBuffer()],
      program.programId
    )
  }

  it("Is initialized!", async () => {
    const data = "marco"
    const [pda] = derivePda();
    // Add your test here.
    await program.methods.add(data).accounts({
      payer: mainKeyPair.publicKey
    }).signers([mainKeyPair]).rpc();

    const account = await program.account.customAccount.fetch(pda);
    assert.equal(account.data, data);
  });

  //for update function 
  // it("is updating", async () => {
  //   const data = "ping"
  //
  // })
});
