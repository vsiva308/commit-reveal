# commit-reveal
Basic Commit + Reveal Voting Program Written on Solana

# How it works
This smart contract uses keccak256 to encrypt a vote with a salt which is kept off the blockchain. When it comes time to commit, the hashed vote is stored on chain. During the reveal process, the user inputs both their vote and salt, which is then matched to the hash using a simple proof to verify the hash.
