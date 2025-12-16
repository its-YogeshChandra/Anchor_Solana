use anchor_lang::prelude::*;

declare_id!("3YqvTusfgzuHKgXFT5XQTntouMT9BszoAEev73wtkfPU");

#[program]
pub mod taskify {
    use super::*;

    pub fn add(ctx: Context<Add>, data: String) -> Result<()> {
        ctx.accounts.pda_account.data = data;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    //function to update the data
    pub fn update(ctx: Context<Update>, new_data: String) -> Result<()> {
        let acc = &mut ctx.accounts.pda_account;
        acc.data = new_data;
        Ok(())
    }

    //function to delete the pda and claim rent
    pub fn data(_ctx: Context<Close>) -> Result<()> {
        //send the msg
        msg!("accounts successfully closed");
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct CustomAccount {
    #[max_len(50)]
    data: String,
}

//create a pda and add data into it
#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer,  space = 8+ CustomAccount::INIT_SPACE, seeds = [b"hello_world",payer.key().as_ref()], bump )]
    pub pda_account: Account<'info, CustomAccount>,
    pub system_program: Program<'info, System>,
}

//for updating the pda data account
#[derive(Accounts)]
pub struct Update<'info> {
    pub signer: Signer<'info>,
    //mutate the account
    #[account(mut , seeds= [b"hello_world", signer.key().as_ref()], bump, 
        )]
    pub pda_account: Account<'info, CustomAccount>,
}

//for deleting the data in the pda
#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
    // the pda account to close
    #[account(mut , seeds= [b"hello_world", receiver.key().as_ref()],
    bump,
    close = receiver)]
    pub pda_account: Account<'info, CustomAccount>,
}

//issue is how to track the pda ()
