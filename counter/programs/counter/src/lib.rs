// imports
use anchor_lang::prelude::*;

// rust macro to declare the program id(key pair) -> command anchor keys list
declare_id!("CG7JHHveoTkJxKrDSmkeR5pidcmpDopkxYwmRwj5xVdq");

#[program] // macro
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(()) // 200

        // setup item layout
    }

    // function to actually add the item
}

#[derive(Accounts)] // macro
pub struct Initialize {} // type declaration