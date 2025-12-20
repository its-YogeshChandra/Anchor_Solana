use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

declare_id!("BaJwzP9zYpBtfFsKvK1YKS8qwAgFc1haABobpgWmUu2R");

#[program]
pub mod tokenmint {

    use super::*;

    //create mint address
    pub fn createmint(ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
        msg!("mint created : {:?}", ctx.accounts.mint.key());
        Ok(())
    }

    //create mint account
    pub fn create_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
        msg!(
            "created token account : {:?}",
            ctx.accounts.token_account.key()
        );
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

//create token account
//done through making a pda derived from and creeated by associated token program.
#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    //check that process is done for the same address who has signed the transaction
    pub signer: Signer<'info>,
    #[account(init_if_needed, payer = signer, associated_token::mint = mint, associated_token::authority = signer, associated_token::token_program = token_program)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

//extras
//mint address :
//from keypair or from program derived address
