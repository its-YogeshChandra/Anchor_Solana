use anchor_lang::prelude::{pubkey::ParsePubkeyError, *};
use anchor_spl::token_interface::{
    self, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
};

declare_id!("4h2c3Fdre59qUXW2keiDhTZy5wbtKgxBT8wZDfsrt3Jd");

#[program]
pub mod vault_program {
    use super::*;

    pub fn initialize(ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    //function to create the token account using the pda through user
    pub fn create_ata(ctx: Context<CreateTokenAccount>) -> Result<()> {
        msg!("token acount : {:?}", ctx.accounts.token_account);
        Ok(())
    }
}

//function to create mint
#[derive(Accounts)]
#[instruction(decimals: u8 )]
pub struct CreateMint<'info> {
    // who gonna pay for this
    #[account(mut)]
    pub payer: Signer<'info>,
    //the macro to initialize the mint process
    #[account( init , payer = payer, mint::decimals = decimals , mint::authority = payer.key(), mint::freeze_authority = payer.key())]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

//function to create ata that is owned by a pda
#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    //payer of the transaction
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,

    // the macro to create the ata
    #[account(init , payer = payer, token::mint = mint, token::authority = token_account ,token::token_program = token_program, seeds = [b"vault", payer.key().as_ref()], bump)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
