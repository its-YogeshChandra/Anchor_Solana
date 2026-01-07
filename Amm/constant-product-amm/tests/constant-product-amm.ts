import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ConstantProductAmm } from "../target/types/constant_product_amm";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, } from "@solana/web3.js";
import { getMint, TOKEN_PROGRAM_ID, getAccount, createAssociatedTokenAccount, createAssociatedTokenAccountInstruction, amountToUiAmountForInterestBearingMintWithoutSimulation, getAssociatedTokenAddress, createSyncNativeInstruction } from "@solana/spl-token"
import { assert } from "chai";
import { BN } from "bn.js";
import { base58 } from "@scure/base";

describe("constant-product-amm", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.constantProductAmm as Program<ConstantProductAmm>;

  //const keypair for user 
  const user_keypair = anchor.web3.Keypair.fromSecretKey(base58.decode("3cJruu3vu3ym1U6dxuYivwyMqrPBnWJFtap26DSMqP56DjjTRpF8mpPMKJgep7o8v9sLhzJCdDLW9vU1PW1r9X9P"))

  //usdc mint address 
  const usdc_mint_address = "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr"
  const sol_mint_address = "So11111111111111111111111111111111111111112"

  const usdcmintPUbkey = new PublicKey(usdc_mint_address)
  const solmintPubkey = new PublicKey(sol_mint_address)

  //make the seed for those 
  let usdc_vault_seed = [Buffer.from("usdc_vault"), usdcmintPUbkey.toBuffer()]
  let sol_vault_seed = [Buffer.from("solana_vault"), solmintPubkey.toBuffer()]
  let pool_state_seed = [Buffer.from("lpstate"), usdcmintPUbkey.toBuffer(), solmintPubkey.toBuffer()]

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
  //first usdc, second  sol, third pool 
  const pdaArr = findPda();
  const poolpda = pdaArr[2];

  // --- HELPER FUNCTION: Safely Get or Create ATA ---
  const getOrCreateATA = async (mint: PublicKey, owner: PublicKey, isWrappedSol = false, solAmount = 0) => {
    const ata = await getAssociatedTokenAddress(mint, owner);
    const tx = new Transaction();
    let shouldSend = false;

    // Check if account exists
    const info = await provider.connection.getAccountInfo(ata);

    if (!info) {
      console.log(`Creating ATA for ${mint.toString().slice(0, 4)}...`);
      tx.add(createAssociatedTokenAccountInstruction(user_keypair.publicKey, ata, owner, mint));
      shouldSend = true;
    }

    // If it's Wrapped SOL, we must transfer SOL and sync
    if (isWrappedSol && solAmount > 0) {
      console.log(`Wrapping ${solAmount} SOL...`);
      tx.add(
        SystemProgram.transfer({
          fromPubkey: user_keypair.publicKey,
          toPubkey: ata,
          lamports: solAmount * LAMPORTS_PER_SOL
        }),
        createSyncNativeInstruction(ata)
      );
      shouldSend = true;
    }

    if (shouldSend) {
      await provider.sendAndConfirm(tx, [user_keypair]);
    }
    return ata;
  };

  it("Is initialized!", async () => {
    // Check if account already exists to prevent test failure on re-run
    const accountInfo = await provider.connection.getAccountInfo(poolpda);
    if (accountInfo !== null) {
      console.log("⚠️ Pool already initialized on Devnet. Skipping init.");
      return;
    }

    // Add your test here.
    //make the pubkey from the usdc and sol mint adress 
    const usdcmintPubkey = new PublicKey(usdc_mint_address);
    const solmintPubkey = new PublicKey(sol_mint_address);

    //check the confirnation of the transaction 
    //make pda from the desired things and check
    //usdc vault seed 
    const usdc_vault_seed = [Buffer.from("usdc_vault"), usdcmintPubkey.toBuffer()];
    const sol_vault_seed = [Buffer.from("solana_vault"), solmintPubkey.toBuffer()];
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

      assert.equal(usdc_vault_account.closeAuthority, pdaArr[2])
      assert.equal(sol_vault_account.closeAuthority, pdaArr[2])
      assert.equal(pool_state_account.owner, SystemProgram.programId)
    }
  }
  )


  ////tranfering function 
  //it("is transfering to vault ", async () => {
  //  const usdcmintPubkey = new PublicKey(usdc_mint_address)
  //  const solmintPubkey = new PublicKey(sol_mint_address)
  //
  //  ////address for user wsol account 
  //  //const userWSolAccount = await getAssociatedTokenAddress(
  //  //  solmintPubkey,
  //  //  user_keypair.publicKey
  //  //)
  //  const userWSolAccount = await getOrCreateATA(solmintPubkey, user_keypair.publicKey, true, 7)
  //
  //
  //  ////create the account 
  //  //const createaccountTX = new Transaction().add(
  //  //  //create the account 
  //  //  createAssociatedTokenAccountInstruction(
  //  //    user_keypair.publicKey,
  //  //    userWSolAccount,
  //  //    user_keypair.publicKey,
  //  //    solmintPubkey
  //  //  ),
  //  //
  //  //  //Move sol into this account 
  //  //  SystemProgram.transfer({
  //  //    fromPubkey: user_keypair.publicKey,
  //  //    toPubkey: userWSolAccount,
  //  //    lamports: 10 * LAMPORTS_PER_SOL
  //  //  }),
  //  //
  //  //  //tell the program to sync the transaction : 
  //  //  createSyncNativeInstruction(userWSolAccount)
  //  //)
  //  //
  //  //await provider.sendAndConfirm(createaccountTX, [user_keypair]);
  //  //
  //
  //  //find the user usdc account 
  //  const userUSDCAccount = await getAssociatedTokenAddress(usdcmintPubkey, user_keypair.publicKey)
  //
  //  //user usdc account not found then  
  //  if (!userUSDCAccount) {
  //    throw console.error("not able to find usdc account")
  //  }
  //
  //  const getuserusdcAccount = await getAccount(provider.connection,
  //    userUSDCAccount,
  //    "confirmed")
  //
  //  console.log('user usdc accont balance : ')
  //  console.log(getuserusdcAccount.amount);
  //
  //  //sol amount 
  //  const sol_amount = new BN(1).mul(new BN(10).pow(new BN(9)));
  //
  //  //usdc amount 
  //  const usdc_amount = new BN(200).mul(new BN(10).pow(new BN(6)));
  //
  //
  //  //call the main function
  //  try {
  //    const transfertx = await program.methods.transferFunction(sol_amount, usdc_amount).accounts({
  //      signer: user_keypair.publicKey,
  //      usdcMint: usdcmintPubkey,
  //      solMint: solmintPubkey,
  //      senderUsdcAccount: userUSDCAccount,
  //      senderSolAccount: userWSolAccount,
  //      tokenprogram: TOKEN_PROGRAM_ID
  //    }).signers([user_keypair]).rpc({ commitment: "confirmed" });
  //
  //    if (transfertx) {
  //
  //      //get account info of the vault account  
  //      const usdc_vault_account = await getAccount(
  //        provider.connection,
  //        pdaArr[0],
  //        "confirmed",
  //        TOKEN_PROGRAM_ID
  //      )
  //      const sol_vault_account = await getAccount(
  //        provider.connection,
  //        pdaArr[1],
  //        "confirmed",
  //        TOKEN_PROGRAM_ID
  //      )
  //      const pool_state_account = await provider.connection.getAccountInfo(pdaArr[2])
  //
  //      if (!usdc_vault_account) {
  //        throw console.error("usdc vault not found ")
  //      }
  //
  //      if (!sol_vault_account) {
  //        throw console.error("sol vault not found ")
  //      }
  //
  //      if (!pool_state_account) {
  //        throw console.error("pool state acount not found ")
  //      }
  //
  //      console.log("usdc_vault_account balance : ")
  //      console.log(usdc_vault_account.amount)
  //
  //      console.log("sol_vault_account balance : ")
  //      console.log(sol_vault_account.amount)
  //
  //    }
  //  } catch (error) {
  //    console.log("failed to add liquidity due to : ")
  //    console.log(error)
  //  }
  //
  //})

  //for swap 
  it("swap is working", async () => {

    //doing swap from usdc to sol
    const userUSDCAccount = await getAssociatedTokenAddress(usdcmintPUbkey, user_keypair.publicKey)

    //user wsol account 
    const userWSolAccountaddress = await getAssociatedTokenAddress(
      solmintPubkey,
      user_keypair.publicKey
    )


    const getuserAccountInfo = await getAccount(provider.connection,
      userUSDCAccount,
      "confirmed"
    )
    console.log("user usdc balance : ")
    console.log(getuserAccountInfo.amount)

    //amount to swap 
    const amount_to_swap = new BN(100).mul(new BN(10).pow(new BN(6)))

    try {
      const swaptx = await program.methods.swapFunction(amount_to_swap).accounts({
        signer: user_keypair.publicKey,
        inputMint: usdcmintPUbkey,
        outputMint: solmintPubkey,
        poolState: pdaArr[2],
        inputVault: pdaArr[0],
        outputVault: pdaArr[1],
        userSourceAccount: userUSDCAccount,
        userDestinationAccount: userWSolAccountaddress,
        tokenProgram: TOKEN_PROGRAM_ID
      }).signers([user_keypair]).rpc({ commitment: "confirmed" })

      //if transaction confirms 
      if (swaptx) {
        //check of the value 
        const wait = () => {
          return new Promise((resolve, reject) => {
            setTimeout(resolve, 5000)
          })
        }

        await wait();

        //check for the amount : 
        const wsolAccount = await getAccount(provider.connection, userWSolAccountaddress, "confirmed")

        //check the sol amount 
        console.log("sol amount :")
        console.log(wsolAccount.amount)
      }

    } catch (error) {
      console.log("failed to swap token due to : ")
      console.error(error)
    }
  })

}
)
