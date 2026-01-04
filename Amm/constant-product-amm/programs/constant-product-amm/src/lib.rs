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
    #[account(mut, seeds= [b"usdc_vault", usdc_mint.key().as_ref()], bump)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    #[account(mut, seeds = [b"solana_vault", sol_mint.key().as_ref()], bump)]
    pub sol_mint: InterfaceAccount<'info, Mint>,

    //sender token accounts(user address)
    #[account(mut)]
    pub sender_usdc_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub sender_sol_account: InterfaceAccount<'info, TokenAccount>,

    //receiver token accounts (vault address)
    #[account(mut)]
    pub receiver_usdc_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_sol_account: InterfaceAccount<'info, TokenAccount>,

    //token program used
    pub tokenprogram: Interface<'info, TokenInterface>,
}

//implement the cpi function on MintTokensp
impl<'info> TransferToVault<'info> {
    pub fn mint_tokens(&self) -> Result<()> {
        //do the cpi inside this
        //calling transfer sol and usdc function
        self.transferusdc()?;
        self.transfersol()?;
        Ok(())
    }

    //for usdc
    fn transferusdc(&self) -> Result<()> {
        //extract decimal and set amount
        let amount = 100;
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
        token_interface::transfer_checked(cpi_context, amount, decimals)?;
        Ok(())
    }

    //for solana
    fn transfersol(&self) -> Result<()> {
        //extract decimal and set amount
        let amount = 3;
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
        token_interface::transfer_checked(cpi_context, amount, decimals)?;
        Ok(())
    }
}

//how to fix the issue that might work
#[derive(Accounts)]
pub struct SwapTokens<'info> {
    //user wallet
    #[account(mut)]
    pub signer: Signer<'info>,

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

impl<'info> SwapTokens<'info> {
    pub fn swap(&self) -> Result<()> {
        //check if the input and output vault matches the pools state
        // let usdc_vault_address = self.pool_state.usdc_vault_address;
        // let sol_vault_address = self.pool_state.sol_vault_address;
        match self.input_vault.key() {
            usdc_vault_address => Ok(()),
            sol_vault_address => Ok(()),
            _ => Ok(()),
        }
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
