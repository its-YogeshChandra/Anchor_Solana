use anchor_lang::prelude::*;

declare_id!("DPan51KtNPYScdj8uDPAraoAXxiM2bdieRpnHSVZPrHT");

#[program]
pub mod sentr_block {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data : Vec<u8>) -> Result<()> {
        println!("initialize working");
        ctx.accounts.pda_account.bump_seed = data;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
   
}


//create the struct for the init function 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer : Signer<'info>, 
    #[account( init, payer = signer, space = 8 + 12, 
        seeds = [b"hello_world", signer.key().as_ref()],
        bump,
        )]
    pub pda_account : Account< 'info , CustomAccount>,
    pub system_program: Program<'info, System>,
}


//custom account type ( containing the bump seed need to make the pda )
#[account]
pub struct CustomAccount {
    pub bump_seed: Vec<u8> ,
}
