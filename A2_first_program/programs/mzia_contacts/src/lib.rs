pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5gGkH3UrsLWEfJ1EZouzpUeH1UZjiRTkaP7aHHLcin5H");

#[program]
pub mod mzia_contacts {
    use super::*;

    /// Save a trusted recipient under a short name (e.g. "deda").
    pub fn add_contact(ctx: Context<AddContact>, name: String, address: Pubkey) -> Result<()> {
        crate::instructions::add_contact::handle_add_contact(ctx, name, address)
    }

    /// Delete a saved contact and refund its rent to the owner.
    pub fn remove_contact(ctx: Context<RemoveContact>) -> Result<()> {
        crate::instructions::remove_contact::handle_remove_contact(ctx)
    }
}
