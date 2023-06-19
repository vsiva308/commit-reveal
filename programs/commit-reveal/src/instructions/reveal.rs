use crate::error::ElectionError;
use crate::state::{Election, voter_record::VoterRecord};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::hashv;

pub fn reveal(
    ctx: Context<Reveal>,
    salt: String,
    _id: u64
) -> Result<()> {
    //verify hash
    let pubkey = ctx.accounts.selection.key();
    let buffer: &[&[u8]] = &[pubkey.as_ref(), &salt.as_bytes()];
    let hash = hashv(buffer);
    msg!("Hash: {:?}", hash.0);
    require!(hash.0 == ctx.accounts.record.vote, ElectionError::HashMismatch);

    //inc an election vec for vote
    //also check if pubkey exists in there
    ctx.accounts.election.increment(&pubkey)?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    salt: String,
    _id: u64
)]
pub struct Reveal<'info> {
    #[account(
        mut,
        seeds = ["election".as_bytes(), _id.to_be_bytes().as_ref()], bump = election.bump
    )]
    pub election: Account<'info, Election>,
    #[account(
        mut,
        close = payer,
        seeds = ["record".as_bytes(), election.key().as_ref(), payer.key().as_ref()], bump = record.bump
    )]
    pub record: Account<'info, VoterRecord>,
    ///CHECK: Done by the keccak hash
    pub selection: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
}
