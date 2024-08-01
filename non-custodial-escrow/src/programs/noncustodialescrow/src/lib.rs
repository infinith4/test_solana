use anchor_lang::prelude::*;

declare_id!("3TBDysJ2uQ7F8JbLAymFKcjwzjzZ3d8acWRcjYUUpURt");

#[program]
pub mod noncustodialescrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
