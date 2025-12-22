use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
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

    //function for minting tokens
    pub fn minttokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        //then main goal is to do cpi
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mintaccount.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program_id = ctx.accounts.mintaccount.to_account_info();
        let cpi_context = CpiContext::new(cpi_program_id, cpi_accounts);
        token_interface::mint_to(cpi_context, amount)?;
        Ok(())
    }

    //function to transfer the token
    pub fn transfertoken(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
        //cpi to tranfer token
        let decimals = ctx.accounts.mint.decimals;
        let cpi_accounts = TransferChecked {
            mint: ctx.accounts.mint.to_account_info().to_account_info(),
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        token_interface::transfer_checked(cpi_context, amount, decimals)?;
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

//mint tokens
#[derive(Accounts)]
pub struct MintTokens<'info> {
    //the mint authority
    #[account(mut)]
    pub signer: Signer<'info>,
    //the acount in which the tokens will get mint into
    #[account(mut)]
    pub mintaccount: InterfaceAccount<'info, Mint>,
    //the destination account
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub tokenprogram: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    //who is sending the token
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    //who is sending the token
    #[account(mut)]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,

    //recipient of the token
    #[account(mut)]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>, // token program thing
}

//extras
//mint address :
//from keypair or from program derived address
