use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::{
    anchor::vrf, instructions::{create_request_randomness_ix, RequestRandomnessParams},
    types::SerializableAccountMeta};

use crate::{ID, instruction};


#[vrf]
#[derive(Accounts)]
pub struct InsideRandVrf<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: done in code
    #[account(mut, address =  ephemeral_vrf_sdk::consts::DEFAULT_EPHEMERAL_QUEUE)]
    pub oracle_queue: AccountInfo<'info>
}

impl<'info>InsideRandVrf<'info> {
    pub fn create_er_rand(&mut self, client_seed: u8) -> Result<()> {
         let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: self.payer.key(),
            oracle_queue: self.oracle_queue.key(),
            callback_program_id: ID,
            callback_discriminator: instruction::Update::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],
            accounts_metas: Some(vec![SerializableAccountMeta { 
                pubkey: self.payer.key(), 
                is_signer: false, 
                is_writable: true
            }]),
            ..Default::default()
        });
        self.invoke_signed_vrf(&self.payer.to_account_info(), &ix)?;

        Ok(())
    }
}
