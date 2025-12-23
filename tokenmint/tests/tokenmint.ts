import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tokenmint } from "../target/types/tokenmint";
import { TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID, getMint } from "@solana/spl-token"
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

describe("tokenmint", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // create keypair 
  const mintkeypair = anchor.web3.Keypair.generate()
  const program = anchor.workspace.tokenmint as Program<Tokenmint>;

  before(
    async () => {
      const sign = await provider.connection.requestAirdrop(mintkeypair.publicKey, 30 * anchor.web3.LAMPORTS_PER_SOL)
      await provider.connection.confirmTransaction(sign, "confirmed")
      const balance = await provider.connection.getBalance(mintkeypair.publicKey)
      console.log(balance)
    }
  )

  it("Is initialized!", async () => {

    // Add your test here.
    const tx = await program.methods.createmint(9).accounts({
      signer: mintkeypair.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID
    }).signers([mintkeypair]).rpc({ commitment: "confirmed" });
    console.log("Your transaction signature", tx);

    //extracting the mint info 
    // const mintAccount = await getMint(
    //  provider.connection,
    //  mintkeypair.publicKey,
    //  "confirmed",
    //  TOKEN_PROGRAM_ID
    //)

    //gain the account info
    const accountinfo = await provider.connection.getAccountInfo(mintkeypair.publicKey)

    console.log("account info  : ")
    console.log(accountinfo.owner)
    console.log("program id : ")
    console.log(TOKEN_PROGRAM_ID)
  })
});
