import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tokenmint } from "../target/types/tokenmint";
import { TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID, getAccount, getAssociatedTokenAddress, getMint, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token"
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import BN from "bn.js"

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

  //to initialized the project 
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.createmint(9).accounts({
      signer: receivermainAccoountkeypair.publicKey,
      mint: mintkeypair.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID
    }).signers([mintkeypair, receivermainAccoountkeypair]).rpc({ commitment: "confirmed" });
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

  //create the ata account for 
  it("is creating ata ", async () => {
    //check the balance first 
    const tx = await program.methods.createAccount().accounts({
      signer: receivermainAccoountkeypair.publicKey,
      mint: mintkeypair.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID
    }).signers([receivermainAccoountkeypair]).rpc({ commitment: "confirmed" });

    //find the ata from the account address 
    const associatedTokenAccount = await getAssociatedTokenAddress(
      mintkeypair.publicKey,
      receivermainAccoountkeypair.publicKey,
      false,
      TOKEN_PROGRAM_ID
    )
    console.log("associatedTokenAccount")
    console.log(associatedTokenAccount)

    //get the account on the ata received 
    const tokenAccount = await getAccount(provider.connection, associatedTokenAccount, "confirmed", TOKEN_PROGRAM_ID)
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

    //token amount
    const tokenAmount = new BN(100)
    const tx = await program.methods.minttokens(tokenAmount).accounts({
      signer: receivermainAccoountkeypair.publicKey,
      tokenAccount: tokenAccount.address,
      mintaccount: mintkeypair.publicKey,
      tokenprogram: TOKEN_PROGRAM_ID,
    }).signers([receivermainAccoountkeypair]).rpc({ commitment: "confirmed" })

    const latestBlockHash = await provider.connection.getLatestBlockhash();

    //wait for transaction to confirm
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });

    // fetch the token account info again :   NOW it is safe to fetch the account
    const tokenAccountInfo = await getAccount(
      provider.connection,
      associatedTokenAccount,
      "confirmed"
    );
    //const token account info to check weahter the token received or not  
    console.log("token account amount : ")
    console.log(tokenAccountInfo.amount.toString())
  }
  )
});
