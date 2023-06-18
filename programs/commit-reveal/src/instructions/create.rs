use crate::state::*;
use anchor_lang::prelude::*;

//initialize a new election
pub fn create(
    ctx: Context<Create>,
    name: String, 
    users: Vec<Pubkey>,
    commit_window: i64, //when commits end
    reveal_window: i64  //when reveals end
) -> Result<()> {
    let bump = *ctx.bumps.get("election").unwrap();
    ctx.accounts.election.initialize(name, users, commit_window, reveal_window, bump)?;
    
    ctx.accounts.counter.count += 1;
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    name: String, 
    users: Vec<Pubkey>,
    commit_window: i64,
    reveal_window: i64
)]
pub struct Create<'info> {
    #[account(
        mut,
        seeds = ["counter".as_bytes()], bump = counter.bump,
    )]
    pub counter: Account<'info, Count>,
    #[account(
        init,
        payer = payer,
        space = Election::get_size(&name, &users),
        seeds = ["election".as_bytes(), counter.count.to_be_bytes().as_ref()], bump
    )]
    pub election: Account<'info, Election>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>
}