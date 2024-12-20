use anchor_lang::prelude::*;

declare_id!("3UWBgy8huXFKtNKv9zpcA8HE2ZfAR2cygTzTj4HLjmcL");

#[program]
pub mod otc_orderbook {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
