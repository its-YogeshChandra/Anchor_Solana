use anchor_lang::prelude::*;

declare_id!("J3gWhe7BHUuVTSg8HNcEEPzaZYTMLL4oHWJtCDjKp4LD");

struct AccontShape {
    pub num : i32
}

pub struct Initialize{
    pub account : Account<AccontShape>
    pub system_program: Program 
}

#[program]
pub mod calccontract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn double(arg: Type) -> RetType {
         todo!();
     }

   pub fn add(arg: Type) -> RetType {
         todo!();
     }

}

