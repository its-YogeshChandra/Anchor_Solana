import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tokenmint } from "../target/types/tokenmint";
import { TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID, getAccount, getAssociatedTokenAddress, getMint, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token"
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("tokenmint", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // create keypair 
  const mintkeypair = anchor.web3.Keypair.generate()
  const program = anchor.workspace.tokenmint as Program<Tokenmint>;

  //function to create the keypair 
  const createKeypair = () => {
    return anchor.web3.Keypair.generate()
  }

  const receivermainAccoountkeypair = anchor.web3.Keypair.generate();
  before(
    async () => {
      const sign = await provider.connection.requestAirdrop(receivermainAccoountkeypair.publicKey, 30 * anchor.web3.LAMPORTS_PER_SOL)
      await provider.connection.confirmTransaction(sign, "confirmed")
      const balance = await provider.connection.getBalance(mintkeypair.publicKey)
      console.log(balance)
    }
  )

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.createmint(9).accounts({
      mint: mintkeypair.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID
    }).signers([mintkeypair]).rpc({ commitment: "confirmed" });
    console.log("Your transaction signature", tx);

    // extracting the mint info
    const mintAccount = await getMint(
      provider.connection,
      mintkeypair.publicKey,
      "confirmed",
      TOKEN_PROGRAM_ID
    )
    // console.log("mint account")
    // console.log(mintAccount)

  })

  it("is creating ata ", async () => {
    //check the balance first 
    const tx = await program.methods.createAccount().accounts({
      signer: receivermainAccoountkeypair.publicKey,
      mint: mintkeypair.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID
    }).signers([receivermainAccoountkeypair]).rpc({ commitment: "confirmed" });
    console.log("transaction : " + tx)

    //find the ata 
    const associatedTokenAccount = await getAssociatedTokenAddress(
      mintkeypair.publicKey,
      receivermainAccoountkeypair.publicKey,
      false,
      TOKEN_PROGRAM_ID
    )

    //get the account on the ata received 
    const tokenAccount = await getAccount(provider.connection, associatedTokenAccount, "confirmed", TOKEN_PROGRAM_ID)
    console.log("token account : " + tokenAccount)
  })

  //mint tokens 
  it("tokens gettting minted ", async () => {
    const associatedTokenAccount = await getAssociatedTokenAddress(
      mintkeypair.publicKey,
      receivermainAccoountkeypair.publicKey,
      false,
      TOKEN_PROGRAM_ID
    )

    //get the account on the ata received 
    const tokenAccount = await getAccount(provider.connection, associatedTokenAccount, "confirmed", TOKEN_PROGRAM_ID)
    console.log("token account : ")
    console.log(tokenAccount)

    const tx = await program.methods.minttokens(100).accounts({
    }).signers().rpc()
  }
  )
});
