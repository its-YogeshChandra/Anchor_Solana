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
  let imppda = []

  before(
    //function to put on the airdrop 
    async () => {
      const sign = await provider.connection.requestAirdrop(mainKeyPair.publicKey, 20 * anchor.web3.LAMPORTS_PER_SOL);
      await provider.connection.confirmTransaction(sign, "confirmed")
    })

  //make the seed Prefix 
  const seedprefix = Buffer.from("hello_world");

  const derivePda = () => {
    //must match you rust seeds exactly 
    return anchor.web3.PublicKey.findProgramAddressSync(
      [seedprefix, mainKeyPair.publicKey.toBuffer()],
      program.programId
    )
  }

  //is initialized 
  it("Is initialized!", async () => {
    const data = "marco"
    const fullpda = derivePda();
    imppda.push(fullpda)
    const [pda] = fullpda;
    console.log('full pda [0]: ')
    console.log(fullpda[0]);
    console.log("pda is : " + pda);
    // Add your test here.
    const tx = await program.methods.add(data).accounts({
      payer: mainKeyPair.publicKey,
    }).signers([mainKeyPair]).rpc();

    const value = await provider.connection.confirmTransaction(tx);
    console.log("value is : ");
    console.log(value);
    const accountinfo = await program.account.customAccount.fetch(pda);
    assert.equal(accountinfo.data, data)
  });

  //for updating 
  it("its updating", async () => {
    //const
    const [pdavalue] = imppda[0]
    const tx = await program.methods.update("figarland").accounts({
      signer: mainKeyPair.publicKey
    }).signers([mainKeyPair]).rpc();

    //value is : 
    const value = await provider.connection.confirmTransaction(tx)
    console.log("value is : ")
    console.log(value)
    const accountdetails = await program.account.customAccount.fetch(pdavalue);
    console.log("account details : ")
    console.log(accountdetails)
    assert.equal(accountdetails.data, "figarland")
  })

  //for deleting
  it("its deleting", async () => {
    //deleting function 
    const tx = await program.methods.data().accounts({
      receiver: mainKeyPair.publicKey,
    }).rpc();

    //find the transaction 
    const value = await provider.connection.confirmTransaction(tx)
    console.log("the value")
    console.log(value)

    const accountdetails = await program.account.customAccount.all();
    console.log("account details")
    console.log(accountdetails)
  })

});
