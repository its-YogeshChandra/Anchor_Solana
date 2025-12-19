use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

declare_id!("BaJwzP9zYpBtfFsKvK1YKS8qwAgFc1haABobpgWmUu2R");

#[program]
pub mod tokenmint {
    use anchor_spl::token_interface::spl_token_metadata_interface::instruction::emit;

    use super::*;

    //create mint address
    pub fn createmint(ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer = signer, mint::decimals = decimals, mint::authority = signer.key(), mint::freeze_authority = signer.key())]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub struct Mint_Authority {
    data: Pubkey,
}
// #[account]
// pub struct DecimalValue {
//     data: u8,
// }
