use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::{anchor::commit, ephem::commit_accounts};

use crate::{state::UserAccount, create_password};

#[commit]
#[derive(Accounts)]
pub struct UpdateCommit<'info> {

    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user", user_account.user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

}

impl<'info> UpdateCommit<'info> {
    
    pub fn update_commit(&mut self, rand_data: [u8; 32]) -> Result<()> {
        
        //let new_data = ephemeral_vrf_sdk::rnd::random_u8_with_range(&rand_data, 1, 255);

        let user_key = self.user_account.user.key().to_bytes();

        let hash = create_password(user_key, rand_data).unwrap();

        // Update the data field
        self.user_account.password = hash;

        commit_accounts(
            &self.user_account.to_account_info(), 
            vec![&self.user_account.to_account_info()], 
            &self.magic_context, 
            &self.magic_program
        )?;
        
        Ok(())
    }
}