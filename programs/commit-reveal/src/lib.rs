use anchor_lang::prelude::*;

use instructions::*;

pub mod state;
pub mod instructions;
pub mod tools;
pub mod error;

declare_id!("79SJezTP7rHbgymPdR8PPehwJ5FDN8fY1Yosybc7R6EQ");

#[program]
pub mod commit_reveal {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize(ctx)
    }

    pub fn create(ctx: Context<Create>, name: String, users: Vec<Pubkey>, commit_window: i64, reveal_window: i64) -> Result<()> {
        instructions::create::create(ctx, name, users, commit_window, reveal_window)
    }

    pub fn commit(ctx: Context<Commit>, id: u64, commitment: [u8; 32]) -> Result<()> {
        instructions::commit::commit(ctx, id, commitment)
    }

    pub fn reveal(ctx: Context<Reveal>, salt: String, id: u64) -> Result<()> { //salt should be 12 chars
        //pass in pubkey as account
        instructions::reveal::reveal(ctx, salt, id)
    }
}