use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

declare_id!("BaJwzP9zYpBtfFsKvK1YKS8qwAgFc1haABobpgWmUu2R");

#[program]
pub mod tokenmint {

    use super::*;

    //create mint address
    pub fn createmint(ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
        msg!("mint created");
        Ok(())
    }
}

//struct for acount
#[derive(Accounts)]
//instruction let one to take input from the arguments itself(layman)
#[instruction(decimals: u8)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer = signer, mint::decimals = decimals, mint::authority = signer.key(), mint::freeze_authority = signer.key())]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// pub struct DecimalValue {
//     data: u8,
// }
