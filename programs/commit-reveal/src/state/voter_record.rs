use crate::tools::anchor::DISCRIMINATOR_SIZE;
use anchor_lang::prelude::*;

#[account]
pub struct VoterRecord {
    pub vote: [u8; 32],
    pub bump: u8
}

impl VoterRecord {
    pub const SIZE: usize = DISCRIMINATOR_SIZE + 32 + 1;
}