use anchor_lang::prelude::*;

use crate::constants::MAX_NAME_LEN;

/// A trusted recipient saved by a wallet owner, e.g. "deda" -> Mom's address.
/// One PDA per (owner, name), so a name is unique within an owner's book.
#[account]
#[derive(InitSpace)]
pub struct Contact {
    pub owner: Pubkey,
    pub address: Pubkey,
    #[max_len(MAX_NAME_LEN)]
    pub name: String,
    pub bump: u8,
}
