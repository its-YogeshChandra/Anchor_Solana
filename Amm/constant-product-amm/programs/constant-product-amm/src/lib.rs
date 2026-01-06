use std::process::Output;

use anchor_lang::{prelude::*, Bump};
use anchor_spl::{
    token,
    token_interface::{self, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
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

    //first function
    pub fn first_function(ctx: Context<CreateLPPoolState>) -> Result<()> {
        //fix the issue
        msg!(" issue : {:?}", ctx.accounts.system_program.key());
        Ok(())
    }

    //transfer function
    pub fn transfer_function(ctx: Context<TransferToVault>) -> Result<()> {
        //tranfervault,
        //call the function from the struct
        ctx.accounts.mint_tokens()?;
        Ok(())
    }

    //swap function
    pub fn swap_function(ctx: Context<SwapTokens>, amount: u64) -> Result<()> {
        //call the function on the struct
        ctx.accounts.init_swap(amount)?;
        //call the cheks function
        ctx.accounts.checks(amount)?;

        //the main swap function
        ctx.accounts.handleswap(amount);

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

    //the mint of the account
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
    #[account(init, payer = signer, space = 8 + LpPoolAccountShape::INIT_SPACE, seeds  = [b"lpstate", usdc_mint.key().as_ref(), sol_mint.key().as_ref()], bump)]
    pub pool_account: Account<'info, LpPoolAccountShape>,
    pub system_program: Program<'info, System>,

    //usdc_vault
    #[account(init, payer= signer, token::mint = usdc_mint, token::authority = pool_account, token::token_program = token_program, seeds = [b"usdc_vault", usdc_mint.key().as_ref()], bump)]
    pub usdc_account: InterfaceAccount<'info, TokenAccount>,

    //wsolana vault
    #[account(init, payer = signer,token::mint = sol_mint, token::authority = pool_account, token::token_program = token_program, seeds = [b"solana_vault", sol_mint.key().as_ref()], bump)]
    pub solana_account: InterfaceAccount<'info, TokenAccount>,
}

//creete function to fill these account
#[derive(Accounts)]
pub struct TransferToVault<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    //has to be changes cause mint mutate or not
    #[account(mut)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub sol_mint: InterfaceAccount<'info, Mint>,

    //sender token accounts(user address)
    #[account(mut)]
    pub sender_usdc_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub sender_sol_account: InterfaceAccount<'info, TokenAccount>,

    //receiver token accounts (vault address)
    #[account(mut, seeds = [b"usdc_vault", usdc_mint.key().as_ref()], bump)]
    pub receiver_usdc_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, seeds = [b"solana_vault", sol_mint.key().as_ref()], bump)]
    pub receiver_sol_account: InterfaceAccount<'info, TokenAccount>,

    //token program used
    pub tokenprogram: Interface<'info, TokenInterface>,
}

//implement the cpi function on MintTokensp
impl<'info> TransferToVault<'info> {
    pub fn mint_tokens(&self, sol_amount: u64, usdc_amount: u64) -> Result<()> {
        //do the cpi inside this
        //calling transfer sol and usdc function
        self.transferusdc(usdc_amount)?;
        self.transfersol(sol_amount)?;
        Ok(())
    }

    //for usdc
    fn transferusdc(&self, usdc_amount: u64) -> Result<()> {
        //extract decimal and set amount
        let decimals = self.usdc_mint.decimals;
        let cpi_accounts = TransferChecked {
            mint: self.usdc_mint.to_account_info(),
            from: self.sender_usdc_account.to_account_info(),
            to: self.receiver_usdc_account.to_account_info(),
            authority: self.signer.to_account_info(),
        };
        //access the token program
        let cpi_program = self.tokenprogram.to_account_info();
        //build cpi context
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        //trandfer token
        token_interface::transfer_checked(cpi_context, usdc_amount, decimals)?;
        Ok(())
    }

    //for solana
    fn transfersol(&self, sol_amount: u64) -> Result<()> {
        //extract decimal and set amount
        let decimals = self.sol_mint.decimals;
        let cpi_accounts = TransferChecked {
            mint: self.sol_mint.to_account_info(),
            from: self.sender_sol_account.to_account_info(),
            to: self.receiver_sol_account.to_account_info(),
            authority: self.signer.to_account_info(),
        };
        //access the token program
        let cpi_program = self.tokenprogram.to_account_info();
        //build cpi context
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        //trandfer token
        token_interface::transfer_checked(cpi_context, sol_amount, decimals)?;
        Ok(())
    }
}

//how to fix the issue that might work
#[derive(Accounts)]
pub struct SwapTokens<'info> {
    //user wallet
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub input_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub output_mint: InterfaceAccount<'info, Mint>,

    //the brain : pool state
    #[account(mut)]
    pub pool_state: Account<'info, LpPoolAccountShape>,

    //input vault
    #[account(mut)]
    pub input_vault: InterfaceAccount<'info, TokenAccount>,

    //output vault
    #[account(mut)]
    pub output_vault: InterfaceAccount<'info, TokenAccount>,

    //user source account
    #[account(mut)]
    pub user_source_account: InterfaceAccount<'info, TokenAccount>,

    //user destination account
    #[account(mut)]
    pub user_destination_account: InterfaceAccount<'info, TokenAccount>,

    //token program
    pub token_program: Interface<'info, TokenInterface>,
}

#[error_code]
pub enum SwapError {
    #[msg("output vault is incorrect")]
    OutputVaultError,
    #[msg("input vault is incorrect")]
    InputVaaultError,
    #[msg("swap amnount is more then account balance")]
    AmountError,
    #[msg("slippag limit exceeds")]
    SlippageError,
}

impl<'info> SwapTokens<'info> {
    pub fn init_swap(&self, amount: u64) -> Result<()> {
        //check if the input and output vault matches the pools state

        //check for the input account if usdc
        if self.input_vault.key() == self.pool_state.usdc_vault_address {
            //thne the output vault address should be of solana
            if self.output_vault.key() != self.pool_state.sol_vault_address {
                //if don't then throw the error
                return err!(SwapError::OutputVaultError);
            }
        } else if self.input_vault.key() == self.pool_state.sol_vault_address {
            //then the output vault should be of usdc
            if self.output_vault.key() != self.pool_state.usdc_vault_address {
                //if don't then throw the error
                return err!(SwapError::OutputVaultError);
                //msg!("output vault is incorrect: {:?}", self.output_vault);
            }
        } else {
            //throw error in the
            return err!(SwapError::InputVaaultError);
            // msg!("input vault wring: {:?}", self.input_vault)
        };

        //for the fix the issue :
        Ok(())
    }

    pub fn checks(&self, amount: u64) -> Result<()> {
        //set the algorithm = constant product
        if !self.user_source_account.amount as u64 > amount {
            //throw error
            return err!(SwapError::AmountError);
        };
        Ok(())
    }

    //fee handler
    pub fn handlefee(&self, amount: u64) {
        //deduct the amount from the user and
    }

    //main swap function
    pub fn handleswap(&self, amount: u64) {
        //call the swap formula
        let output_amount = self.swap_calculation(amount);

        //transfer amount to input vault
        let _ = self.tranferamount(output_amount, amount as u64);
    }

    //swap formula
    fn swap_calculation(&self, input_amount: u64) -> u128 {
        //fix the issue;
        let input_vaultamount = self.input_vault.amount as u128;
        let output_vaultamount = self.output_vault.amount as u128;

        //product before swap
        let product_before_swap = (input_vaultamount * output_vaultamount) as u128;

        //formula to calculate output amount
        let outputamount = (output_vaultamount * input_amount as u128)
            / (input_vaultamount + input_amount as u128);
        outputamount as u128
    }

    //function that transfering amount
    fn tranferamount(&self, output_amount: u128, amounttoswap: u64) -> Result<()> {
        //extract decimal and set amount
        let amount = amounttoswap;
        let decimals = self.input_mint.decimals;
        let outputdecimals = self.output_mint.decimals;

        //make cpi context for the input tranfer
        let cpi_accounts = TransferChecked {
            mint: self.input_mint.to_account_info(),
            from: self.user_source_account.to_account_info(),
            to: self.input_vault.to_account_info(),
            authority: self.signer.to_account_info(),
        };
        //access the token program
        let cpi_program = self.token_program.to_account_info();
        //build cpi context
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        //trandfer token to vault
        token_interface::transfer_checked(cpi_context, amount, decimals)?;

        //seed for the pool
        let seeds = [
            b"lpstate",
            self.pool_state.usdc_mint.as_ref(),
            self.pool_state.sol_mint.as_ref(),
            &[self.pool_state.bump],
        ];

        //the main signer seed
        let signer_seeds = &[&seeds[..]];

        //make cpi context for the output transfer
        let output_cpi_accounts = TransferChecked {
            mint: self.output_mint.to_account_info(),
            from: self.output_vault.to_account_info(),
            to: self.user_destination_account.to_account_info(),
            authority: self.pool_state.to_account_info(),
        };

        //program and cpi context for output
        let output_cpi_program = self.token_program.to_account_info();
        let output_cpi_context =
            CpiContext::new_with_signer(output_cpi_program, output_cpi_accounts, signer_seeds);

        //transfer token from vault to user account
        token_interface::transfer_checked(
            output_cpi_context,
            output_amount as u64,
            outputdecimals,
        )?;

        Ok(())
    }
}

// /Paragraph::new(active_text.as_str())
//             .block(Block::bordered().title(" Detailed Input (Press 'e' to Edit) ").border_style(Style::default().fg(Color::Yellow)))
//             .render(chunks[1], buf);/deriving function
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
