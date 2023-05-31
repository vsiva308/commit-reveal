use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod commit_reveal {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, users: Vec<Pubkey>) -> Result<()> {
        //require!(user.len() <= 5, "error here");
        Ok(())
    }

    pub fn commit(ctx: Context<Commit>, hash: String, id: u64) -> Result<()> {
        Ok(())
    }

    pub fn reveal(ctx: Context<Reveal>, salt: String, id: u64) -> Result<()> { //salt should be 12 chars
        //pass in pubkey as account
        Ok(())
    }
}

#[account]
pub struct VoterRecord {
    vote: String
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(hash: String, id: u64)]
pub struct Commit<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(
        init,
        space = hash.len() + 4,
        payer = voter,
    )]
    pub vote_pda: Account<'info, VoterRecord>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Reveal<'info> {
    pub payer: Signer<'info>
}
