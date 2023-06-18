use crate::state::*;
use anchor_lang::prelude::*;

pub fn initialize(
    ctx: Context<Initialize>
) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    let bump = *ctx.bumps.get("counter").unwrap();
    counter.set_inner(Count {
        count: 0,
        bump
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = Count::SIZE,
        seeds = ["counter".as_bytes()],
        bump
    )]
    pub counter: Account<'info, Count>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>
}