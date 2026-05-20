use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = UserAccount::INIT_SPACE,
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitUser<'info> {
    pub fn initialize(&mut self, bumps: &InitUserBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount { 
            user: *self.user.key, 
            password: [0u8; 32],
            bump: bumps.user_account 
        });
        
        Ok(())
    }
}