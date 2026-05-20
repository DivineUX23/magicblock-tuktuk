use anchor_lang::prelude::*;

use crate::{state::UserAccount, create_password};

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,

}

impl<'info> UpdateUser<'info> {
    pub fn update(&mut self, rand_data: [u8; 32]) -> Result<()> {
        //let new_data = ephemeral_vrf_sdk::rnd::random_u8_with_range(&rand_data, 1, 255);

        let user_key = self.user_account.user.key().to_bytes();

        let hash = create_password(user_key, rand_data).unwrap();

        // Update the data field
        self.user_account.password = hash;
        
        Ok(())
    }
}