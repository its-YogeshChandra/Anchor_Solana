use anchor_lang::{prelude::*, Bump};
use anchor_spl::token_interface::{
    self, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
};

//declare id
declare_id!("HD5S5WBt7U6N4XLdYe1cZu5sYN8Y7uJepsrr7eGX8DYJ");

#[program]
pub mod constant_product_amm {

    use super::*;

    pub fn initialize(ctx: Context<LpTokenMint>) -> Result<()> {
        let mintpda = msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

//function to create mint
#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct LpTokenMint<'info> {
    //who will pay for the mint
    #[account[mut]]
    pub signer: Signer<'info>,

    //init the accont
    #[account(init, payer = signer, mint::decimals = decimals, mint::authority = signer.key(), mint::freeze_authority = signer.key())]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

//function to create ata that is owned by a pda
#[derive(Accounts)]
pub struct CreateLpTokenAccount<'info> {
    //who gonna pay for this
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,

    //init the account
    #[account(init, payer= signer, token::mint = mint, token::authority = token_account, token::token_program = token_program, seeds = [b"vault", signer.key().as_ref()],bump )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

//function to make token mint
#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    //the mint of the acount
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    // the receiver account
    #[account(mut)]
    pub token_acount: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

//lp pool

//account shape for the pool state
#[account]
#[derive(InitSpace)]
pub struct LpPoolAccountShape {
    pub usdc_mint: Pubkey,
    pub sol_mint: Pubkey,
    pub usdc_vault_address: Pubkey,
    pub sol_vault_address: Pubkey,
    pub lp_token_mint: Pubkey,
    pub bump: u8,
}

//deriving function
//two ways: (i) seperate function, (ii) single function
//(i)seperate function
// #[derive(Accounts)]
// pub struct CreateLPPoolState<'info> {
//     //init a data account on a pda
//     #[account(mut)]
//     pub signer: Signer<'info>,
//     #[account(init , payer = signer, space = 8+LpPoolAccountShape::INIT_SPACE, seeds = [b"lpstate", signer.key().as_ref()], bump)]
//     pub pool_account: Account<'info, LpPoolAccountShape>,
//     pub system_program: Program<'info, System>,
// }
//
// //usdc vault
// #[derive(Accounts)]
// pub struct UsdcVault<'info> {
//     #[account(mut)]
//     pub signer: Signer<'info>,
//     pub mint: InterfaceAccount<'info, Mint>,
//     //owner of this account
//     pub pool_state: Account<'info, LpPoolAccountShape>,
//
//     //the macro to create  the ata
//     #[account(init, payer = signer, token::mint = mint, token::authority = pool_state, token::token_program = token_program, seeds = [b"usdc_vault", mint.key().as_ref()], bump)]
//     pub token_account: InterfaceAccount<'info, TokenAccount>,
//     pub token_program: Interface<'info, TokenInterface>,
//     pub system_program: Program<'info, System>,
// }
//
// //sol vault
// #[derive(Accounts)]
// pub struct SolVault<'info> {
//     #[account(mut)]
//     pub signer: Signer<'info>,
//     pub mint: InterfaceAccount<'info, Mint>,
//
//     //owner of the this account
//     pub pool_state: Account<'info, LpPoolAccountShape>,
//     //create ata
//     #[account(init, payer= signer, token::mint = mint, token::authority = pool_state, token::token_program = token_program, seeds = [b"solana_vault", mint.key().as_ref()], bump)]
//     pub token_account: InterfaceAccount<'info, TokenAccount>,
//     pub token_program: Interface<'info, TokenInterface>,
//     pub system_program: Program<'info, System>,
// }

//single function
#[derive(Accounts)]
pub struct CreateLPPoolState<'info> {
    //payer of the account
    #[account(mut)]
    pub signer: Signer<'info>,
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub sol_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,

    //create the account first
    #[account(init, payer = signer, space = 8 + LpPoolAccountShape::INIT_SPACE, seeds  = [b"lpstate", signer.key().as_ref()], bump)]
    pub pool_account: Account<'info, LpPoolAccountShape>,
    pub system_program: Program<'info, System>,

    //usdc_vault
    #[account(init, payer= signer, token::mint = usdc_mint, token::authority = pool_account, token::token_program = token_program, seeds = [b"usdc_vault", usdc_mint.key().as_ref()], bump)]
    pub usdc_account: InterfaceAccount<'info, TokenAccount>,

    //solana vault
    #[account(init, payer = signer,token::mint = sol_mint, token::authority = pool_account, token::token_program = token_program, seeds = [b"solana_vault", sol_mint.key().as_ref()], bump)]
    pub solana_account: InterfaceAccount<'info, TokenAccount>,
}
