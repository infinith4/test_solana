use anchor_lang::prelude::*;

declare_id!("HmmTDoAmbNzXgXVCcHVrndGAVNSg6Ka2QjbZHF2oGrE9");

#[program]
pub mod hello_world {
    use super::*;
    pub fn hello_world(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world, from solana smart contract");
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize {}
