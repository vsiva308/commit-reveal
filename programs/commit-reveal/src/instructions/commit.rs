use crate::state::{Election, voter_record::VoterRecord};
use anchor_lang::prelude::*;

pub fn commit(
    ctx: Context<Commit>,
    _id: u64,
    commitment: [u8; 32]
) -> Result<()> {
    //verify timestamps
    ctx.accounts.election.can_commit()?;
    let record = &mut ctx.accounts.record;
    let bump = *ctx.bumps.get("record").unwrap();
    record.set_inner(VoterRecord {
        vote: commitment,
        bump
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    _id: u64,
    commitment: [u8; 32]
)]
pub struct Commit<'info> {
    #[account(
        seeds = ["election".as_bytes(), _id.to_be_bytes().as_ref()], bump = election.bump
    )]
    pub election: Account<'info, Election>,
    #[account(
        init,
        payer = payer,
        space = VoterRecord::SIZE,
        seeds = ["record".as_bytes(), election.key().as_ref(), payer.key().as_ref()], bump
    )]
    pub record: Account<'info, VoterRecord>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>
}