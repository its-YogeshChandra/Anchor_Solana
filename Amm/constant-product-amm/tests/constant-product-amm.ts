import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ConstantProductAmm } from "../target/types/constant_product_amm";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, } from "@solana/web3.js";
import { getMint, TOKEN_PROGRAM_ID, getAccount, createAssociatedTokenAccount, createAssociatedTokenAccountInstruction, amountToUiAmountForInterestBearingMintWithoutSimulation, getAssociatedTokenAddress, createSyncNativeInstruction } from "@solana/spl-token"
import { assert } from "chai";

describe("constant-product-amm", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.constantProductAmm as Program<ConstantProductAmm>;

  //const keypair for user 
  const encoder = new TextEncoder();
  const user_keypair = anchor.web3.Keypair.fromSecretKey(encoder.encode("3cJruu3vu3ym1U6dxuYivwyMqrPBnWJFtap26DSMqP56DjjTRpF8mpPMKJgep7o8v9sLhzJCdDLW9vU1PW1r9X9P"))

  //usdc mint address 
  const usdc_mint_address = "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr"
  const sol_mint_address = "So11111111111111111111111111111111111111112"

  // before(async () => {
  //   await provider.connection.requestAirdrop(user_keypair.publicKey, )
  // })

  it("Is initialized!", async () => {
    // Add your test here.
    //make the pubkey from the usdc and sol mint adress 
    const usdcmintPubkey = new PublicKey(usdc_mint_address);
    const solmintPubkey = new PublicKey(sol_mint_address);

    //check the confirnation of the transaction 
    //make pda from the desired things and check
    //usdc vault seed 
    const usdc_vault_seed = [Buffer.from("usdc_vault"), usdcmintPubkey.toBuffer()];
    const sol_vault_seed = [Buffer.from("sol_vault"), solmintPubkey.toBuffer()];
    const pool_state_seed = [Buffer.from("lpstate"), usdcmintPubkey.toBuffer(), solmintPubkey.toBuffer()];

    const seedArray = [usdc_vault_seed, sol_vault_seed, pool_state_seed]

    const findPda = () => {
      //make the returning object 
      const pdaArr = []

      //map the array and find pda 
      seedArray.map(e => {
        const [pda, bump] = PublicKey.findProgramAddressSync(e, program.programId)
        pdaArr.push(pda)
      })
      return pdaArr
    }
    const pdaArr = findPda();
    const usdc_mint_account = await getMint(
      provider.connection,
      usdcmintPubkey,
      "confirmed"
    )

    if (!usdc_mint_address) {
      throw console.error("error while fetching usdc mint ");
    }


    //call the function 
    const tx = await program.methods.firstFunction().accounts({
      signer: user_keypair.publicKey,
      solMint: solmintPubkey,
      usdcMint: usdcmintPubkey,
      tokenProgram: TOKEN_PROGRAM_ID
    }).signers([user_keypair]).rpc({ commitment: "confirmed" })

    //check the accounts are made or not 
    if (tx) {
      //check the account info 
      const usdc_vault_account = await getAccount(
        provider.connection,
        pdaArr[0],
        "confirmed",
        TOKEN_PROGRAM_ID
      )
      const sol_vault_account = await getAccount(
        provider.connection,
        pdaArr[1],
        "confirmed",
        TOKEN_PROGRAM_ID
      )
      const pool_state_account = await provider.connection.getAccountInfo(pdaArr[2])

      if (!usdc_vault_account) {
        throw console.error("usdc vault not found ")
      }

      if (!sol_vault_account) {
        throw console.error("sol vault not found ")
      }

      if (!pool_state_account) {
        throw console.error("pool state acount not found ")
      }

      assert.equal(usdc_vault_account.closeAuthority, pdaArr[3])
      assert.equal(sol_vault_account.closeAuthority, pdaArr[3])
      assert.equal(pool_state_account.owner, SystemProgram.programId)
    }
  }
  )

  it("is transfering to vault ", async () => {
    const usdcmintPubkey = new PublicKey(usdc_mint_address)
    const solmintPubkey = new PublicKey(sol_mint_address)

    //address for user wsol account 
    const userWSolAccount = await getAssociatedTokenAddress(
      solmintPubkey,
      user_keypair.publicKey
    )

    //create the account 
    const createaccountTX = new Transaction().add(
      //create the account 
      createAssociatedTokenAccountInstruction(
        user_keypair.publicKey,
        userWSolAccount,
        user_keypair.publicKey,
        solmintPubkey
      ),
      //Move sol into this account 
      SystemProgram.transfer({
        fromPubkey: user_keypair.publicKey,
        toPubkey: userWSolAccount,
        lamports: 10 * LAMPORTS_PER_SOL
      }),

      //tell the program to sync the transaction : 
      createSyncNativeInstruction(userWSolAccount)
    )

    await provider.sendAndConfirm(createaccountTX);

    //find the user usdc account 
    const userUSDCAccount = await getAssociatedTokenAddress(usdcmintPubkey, user_keypair.publicKey)

    //user usdc account not found then  
    if (!userUSDCAccount) {
      throw console.error("not able to find usdc account")
    }
  })



}
)
