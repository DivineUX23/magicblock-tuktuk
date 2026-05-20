use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub user: Pubkey,
    pub password: [u8; 32], 
    pub bump: u8,
}

impl Space for UserAccount {
    const INIT_SPACE: usize = 32 + 32 + 1 + 8; // Pubkey + u64 + u8 + 8 bytes for account discriminator
}