use anchor_lang::prelude::*;

declare_id!("EkccquhsLjUjzm77RaRp9SDSSmSgvwkQJoHCKqartGGv");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
