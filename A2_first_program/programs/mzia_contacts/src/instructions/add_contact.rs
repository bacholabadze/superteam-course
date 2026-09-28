use anchor_lang::prelude::*;

use crate::{constants::*, error::ErrorCode, state::Contact};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct AddContact<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Contact::INIT_SPACE,
        seeds = [CONTACT_SEED, owner.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub contact: Account<'info, Contact>,
    pub system_program: Program<'info, System>,
}

pub fn handle_add_contact(ctx: Context<AddContact>, name: String, address: Pubkey) -> Result<()> {
    require!(
        !name.is_empty() && name.len() <= MAX_NAME_LEN,
        ErrorCode::InvalidName,
    );
    require_keys_neq!(address, ctx.accounts.owner.key(), ErrorCode::SelfContact);

    let contact = &mut ctx.accounts.contact;
    contact.owner = ctx.accounts.owner.key();
    contact.address = address;
    contact.name = name;
    contact.bump = ctx.bumps.contact;

    msg!("Mzia: saved trusted contact '{}' -> {}", contact.name, contact.address);
    Ok(())
}
