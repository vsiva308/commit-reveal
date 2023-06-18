use crate::{tools::anchor::DISCRIMINATOR_SIZE, error::ElectionError}; 
use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};

#[account]
pub struct Election {
    name: String,
    users: Vec<Pubkey>,
    votes: Vec<u8>,
    commit_window: i64, //timestamps
    reveal_window: i64,
    pub bump: u8,
}

impl Election {
    const FIVE_MIN: i64 = 300; //5*60

    pub fn initialize(&mut self, name: String, users: Vec<Pubkey>, commit_window: i64, reveal_window: i64, bump: u8) -> Result<()> {
        //doesn't deal w duplicate pubkeys :|
        let timestamp = Clock::get().unwrap().unix_timestamp;
        require!(name.len() < 15, ElectionError::NameTooLong);
        require!(users.len() > 1, ElectionError::NotEnoughUsers);
        require!(commit_window >= timestamp + Self::FIVE_MIN && reveal_window >= commit_window + Self::FIVE_MIN, ElectionError::TimeConstraintViolation);
        let zeros = std::iter::repeat(0).take(users.len());

        self.name = name;
        self.users = users;
        self.votes = Vec::from_iter(zeros);
        self.commit_window = commit_window;
        self.reveal_window = reveal_window;
        self.bump = bump;

        Ok(())
    }

    pub fn increment(&mut self, pubkey: &Pubkey) -> Result<()> {
        let timestamp = Clock::get().unwrap().unix_timestamp;
        require!(timestamp <= self.reveal_window && timestamp > self.commit_window, ElectionError::RevealEnded);
        match self.users.iter().position(|&key| key == *pubkey) {
            Some(x) => {
                self.votes[x] += 1;
                Ok(())
            },
            None => err!(ElectionError::InvalidPubkey)
        }
    }

    pub fn get_size(name: &String, users: &Vec<Pubkey>) -> usize {
        DISCRIMINATOR_SIZE + 4 + name.len() + 4 + (users.len() * PUBKEY_BYTES) + 4 + users.len() + 8 + 8 + 1
    }

    pub fn can_commit(&self) -> Result<()> {
        let timestamp = Clock::get().unwrap().unix_timestamp;
        require!(timestamp <= self.commit_window, ElectionError::CommitEnded);
        Ok(())
    }
}