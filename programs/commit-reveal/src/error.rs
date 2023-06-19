use anchor_lang::error_code;

#[error_code]
pub enum ElectionError {
    #[msg("Election must contain at least two users to vote for.")]
    NotEnoughUsers,
    #[msg("Minimum five minute commit / reveal. Time constraint not met.")]
    TimeConstraintViolation,
    #[msg("Name too long. Limit 16 chars.")]
    NameTooLong,
    #[msg("Commit window is over.")]
    CommitEnded,
    #[msg("Hash does not match.")]
    HashMismatch,
    #[msg("Voted for invalid public key.")]
    InvalidPubkey,
    #[msg("Reveal window is over.")]
    RevealEnded,
    #[msg("Reveal window not started.")]
    RevealNotStarted
}