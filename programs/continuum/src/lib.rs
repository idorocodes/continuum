use anchor_lang::prelude::*;

declare_id!("65cWheNQeRqzi2WWWr5RYTLuW1xkxmp8QEWBuBFzPnvN");

#[program]
pub mod continuum {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
