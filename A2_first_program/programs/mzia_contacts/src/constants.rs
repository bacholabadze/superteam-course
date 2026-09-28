use anchor_lang::prelude::*;

#[constant]
pub const CONTACT_SEED: &[u8] = b"contact";

/// A PDA seed can be at most 32 bytes, so the contact name is capped at 32.
pub const MAX_NAME_LEN: usize = 32;
