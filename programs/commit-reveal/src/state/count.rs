use crate::tools::anchor::DISCRIMINATOR_SIZE;
use anchor_lang::prelude::*;

#[account]
pub struct Count {
    pub count: u64,
    pub bump: u8
}

impl Count {
    pub const SIZE: usize = DISCRIMINATOR_SIZE + 8 + 1;
}