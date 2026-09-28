use anchor_lang::prelude::*;

use crate::{constants::*, state::Contact};

#[derive(Accounts)]
pub struct RemoveContact<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // Seeds re-derive the PDA from the signer, so only its owner can close it.
    // `close` wipes the account and refunds its rent to the owner.
    #[account(
        mut,
        close = owner,
        has_one = owner,
        seeds = [CONTACT_SEED, owner.key().as_ref(), contact.name.as_bytes()],
        bump = contact.bump
    )]
    pub contact: Account<'info, Contact>,
}

pub fn handle_remove_contact(ctx: Context<RemoveContact>) -> Result<()> {
    msg!("Mzia: removed trusted contact '{}'", ctx.accounts.contact.name);
    Ok(())
}
