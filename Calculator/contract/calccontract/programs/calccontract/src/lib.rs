use anchor_lang::prelude::*;

declare_id!("J3gWhe7BHUuVTSg8HNcEEPzaZYTMLL4oHWJtCDjKp4LD");

#[program]
pub mod class_cal {
    //bringing imports from the global scope
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.account.data = 0;
        Ok(())
    }

    pub fn double(ctx: Context<Double>) -> Result<()> {
        ctx.accounts.accounts.data = ctx.accounts.accounts.data * 2;
        Ok(())
    }

    pub fn add(ctx: Context<Add>, amount: i32) -> Result<()> {
        ctx.accounts.accounts.data = ctx.accounts.accounts.data + amount;
        Ok(())
    }
}

#[account]
struct AccountShape {
    data: i32,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account (init, payer=signer, space = 8+4)]
    account: Account<'info, AccountShape>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(mut)]
    accounts: Account<'info, AccountShape>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    accounts: Account<'info, AccountShape>,
    pub signer: Signer<'info>,
}
