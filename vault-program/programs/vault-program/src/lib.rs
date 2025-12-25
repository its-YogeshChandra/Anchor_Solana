use anchor_lang::prelude::*;

declare_id!("4h2c3Fdre59qUXW2keiDhTZy5wbtKgxBT8wZDfsrt3Jd");

#[program]
pub mod vault_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
