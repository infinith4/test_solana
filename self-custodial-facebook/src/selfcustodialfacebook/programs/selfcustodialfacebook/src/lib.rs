use anchor_lang::prelude::*;

declare_id!("EFEL7YPBM7r8VVtmNre8Qs2sXAAfusDMSarspGcqeJG5");

#[program]
pub mod selfcustodialfacebook {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
