import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VaultProgram } from "../target/types/vault_program";
import { LAMPORTS_PER_SOL, PUBLIC_KEY_LENGTH, PublicKey, } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, getMint, getAccount } from "@solana/spl-token";
describe("vault-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const mintKeyPair = anchor.web3.Keypair.generate();
  const userWallet = anchor.web3.Keypair.generate();
  const program = anchor.workspace.vaultProgram as Program<VaultProgram>;

  before(async () => {
    const airdrpSig = await provider.connection.requestAirdrop(userWallet.publicKey, 30 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(airdrpSig)
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(9).accounts({
      payer: userWallet.publicKey,
      mint: mintKeyPair.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID
    }).signers([mintKeyPair, userWallet]).rpc({ commitment: "confirmed" });

    //find the mint account from the user userWallet 
    const mintAccount = await getMint(provider.connection,
      mintKeyPair.publicKey,
      "confirmed",
      TOKEN_PROGRAM_ID)

    console.log("mint Account : ")
    console.log(mintAccount)

    console.log("mint publickey: ")
    console.log(mintKeyPair.publicKey)
  });


  // create ata using the pda 
  it("ata is creating ", async () => {
    //create the seed
    const seed = [Buffer.from("vault"), userWallet.publicKey.toBuffer()]

    //function to find the pda 
    const [vaultPDA, vaultBump] = PublicKey.findProgramAddressSync(seed, program.programId)

    //add the user to it 
    const tx = await program.methods.createAta().accountsPartial({
      payer: userWallet.publicKey,
      mint: mintKeyPair.publicKey,
      tokenAccount: vaultPDA,
      tokenProgram: TOKEN_PROGRAM_ID
    }).signers([userWallet]).rpc({ commitment: "confirmed" })

    //add the user to it : 
    console.log(tx)
  })

});
