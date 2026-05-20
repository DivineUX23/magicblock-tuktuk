use anchor_lang::{prelude::*, solana_program::keccak};


pub fn create_password (user: [u8; 32], rand_data: [u8; 32]) -> Result<[u8; 32]> {

        let user_key = user;
        let timestamp = Clock::get()?.unix_timestamp.to_le_bytes();

        let mut data = Vec::new();

        data.extend_from_slice(&user_key);
        data.extend_from_slice(&timestamp);
        data.extend_from_slice(&rand_data);

        let hash = keccak::hash(&data);

        // Update the data field
        Ok(hash.to_bytes())

}