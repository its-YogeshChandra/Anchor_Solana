import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Taskify } from "../target/types/taskify";
import { assert } from "chai";

describe("taskify", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.taskify as Program<Taskify>;
  const mainKeyPair = anchor.web3.Keypair.generate();

  before(
    //function to put on the airdrop 
    async () => {
      const sign = await provider.connection.requestAirdrop(mainKeyPair.publicKey, 20 * anchor.web3.LAMPORTS_PER_SOL);
      await provider.connection.confirmTransaction(sign, "confirmed")
    })

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
    console.log("pda is : " + pda);
    // Add your test here.
    const tx = await program.methods.add(data).accounts({
      payer: mainKeyPair.publicKey,
    }).signers([mainKeyPair]).rpc();

    const value = await provider.connection.confirmTransaction(tx);
    console.log("value is : ");
    console.log(value);
    const account = await program.account.customAccount.all();
    console.log("account: ");
    console.log(account);
  });


  //for updating 
  // it("is updating", async () => {
  //
  //   const accounts = await provider.connection.getProgramAccounts(mainKeyPair.publicKey)
  //   console.log(accounts);
  //
  //   const pda = "9GxLZnyNjK5hkUuGzPQFbYTQzpXfq2pZGBLASktFu6L4"
  //   // await the program 
  //   await program.methods.update("figarland").accounts({
  //     signer: pda
  //   }).signers([mainKeyPair]).rpc();
  //   const account = await program.account.customAccount.fetch(pda)
  //
  //   //assert 
  //   assert.equal(account.data, "figarland");
  // })


});
