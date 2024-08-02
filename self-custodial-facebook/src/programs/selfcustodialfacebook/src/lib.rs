use anchor_lang::prelude::*;

declare_id!("EFEL7YPBM7r8VVtmNre8Qs2sXAAfusDMSarspGcqeJG5");

#[program]
pub mod self_custodial_facebook {
    use super::*;

    pub fn create_facebook(ctx: Context<Initialize>, name: String, status: String, twitter: String) -> Result<()> {
        let users_account_data = &mut ctx.accounts.facebook_account;
        initialize_account(users_account_data, ctx.bumps.facebook_account, ctx.accounts.signer.key, &name, &status, &twitter)?;
        log_account_creation(&name, &status, &twitter);
        Ok(())
    }

    pub fn update_status(ctx: Context<Update>, new_status: String) -> Result<()> {
        // Update user status, much more like whatsapp 24 hour status, without self destruction 😉
        msg!("Updating status from :: {0} -> to :: {1}", &ctx.accounts.facebook_account.status, &new_status);
        ctx.accounts.facebook_account.status = new_status;

        Ok(())
    }

    pub fn delete_account(_ctx: Context<Close>) -> Result<()> {
        msg!("Account Closed successfully");
        Ok(())
    }
}

fn initialize_account(account: &mut Account<FacebookAccount>, bump: u8, authority: &Pubkey, name: &str, status: &str, twitter: &str) -> Result<()> {
    account.bump = bump;
    account.authority = *authority;
    account.name = name.to_string();
    account.status = status.to_string();
    account.twitter = twitter.to_string();
    Ok(())
}

fn log_account_creation(name: &str, status: &str, twitter: &str) {
    msg!("Created a new account with following details 
        Name :: {0}
        Status :: {1}
        Twitter :: {2}
        ", name, status, twitter 
    );
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // User's account
    #[account(mut)]
    pub signer: Signer<'info>,
    // Creating a new account for every user with seed of their wallet address.
    // This constraint allow one-account per wallet address
    #[account(
        init, 
        payer = signer, 
        space = FacebookAccount::LEN, 
        seeds = ["self-custodial-facebook2".as_bytes(), signer.key().as_ref()], 
        bump,
    )] 
    pub facebook_account: Box<Account<'info, FacebookAccount>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // user's facebook account    
    #[account(
        mut,
        seeds = ["self-custodial-facebook".as_bytes(), signer.key().as_ref()], 
        bump = facebook_account.bump,
    )]
    pub facebook_account: Box<Account<'info, FacebookAccount>>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // we will use `close` for closing user's facebook account.
    #[account(
        mut,
        seeds = ["self-custodial-facebook".as_bytes(), signer.key().as_ref()], 
        bump = facebook_account.bump,
        close=signer
    )]
    pub facebook_account: Box<Account<'info, FacebookAccount>>,
}

#[account]
pub struct FacebookAccount {
    pub authority: Pubkey, // Authority of this account
    pub bump: u8,
    pub name: String, // Max 10 Chars
    pub status: String, // Max 50 Chars
    pub twitter: String // Max 10 Chars
}

impl FacebookAccount {
    const LEN: usize = 
         8 + // discriminator
        32 + // Pubkey
        1 + // bump
        (4 + 10) + // 10 chars of Name
        (4 + 50) + // 50 chars of Status  
        (4 + 10); // 10 chars' twitter
}