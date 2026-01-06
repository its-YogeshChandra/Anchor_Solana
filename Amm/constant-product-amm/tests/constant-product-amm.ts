import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ConstantProductAmm } from "../target/types/constant_product_amm";

describe("constant-product-amm", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.constantProductAmm as Program<ConstantProductAmm>;

  //const keypair for user 
  const user_keypair = anchor.web3.Keypair.generate();

  //usdc mint address 
  const usdc_mint_address = "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr"
  const sol_mint_address = "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr"

  // before(async () => {
  //   await provider.connection.requestAirdrop(user_keypair.publicKey, )
  // })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
