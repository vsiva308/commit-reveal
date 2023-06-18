import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { CommitReveal } from "../target/types/commit_reveal";
import { expect } from "chai";

describe("commit-reveal", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CommitReveal as Program<CommitReveal>;
  const provider = program.provider as anchor.AnchorProvider;

  it("create!", async () => {
    let keyOne = Keypair.generate().publicKey;
    let keyTwo = Keypair.generate().publicKey;

    console.log("KeyOne: ", keyOne.toString(), "\nKeyTwo: ", keyTwo.toString());
    const signer = provider.wallet;

    const [counterPDA, ] = PublicKey
      .findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("counter")
        ],
        program.programId
      );

    let counterState = await program.account.count.fetch(counterPDA);

    const [electionPDA, ] = PublicKey
        .findProgramAddressSync(
            [
                anchor.utils.bytes.utf8.encode("election"),
                counterState.count.toArrayLike(Buffer, "be", 8)
            ],
            program.programId
        );

    let cur_time = Math.floor(Date.now() / 1000);

    const tx = 
        await program.methods.create(
            "Varun",
            [keyOne, keyTwo, PublicKey.default, signer.publicKey],
            new anchor.BN(cur_time + 8*60), //8 min commit
            new anchor.BN(cur_time + 20*60), //12 min reveal
        )
        .accounts({
          counter: counterPDA,
          election: electionPDA,
          payer: signer.publicKey,
          systemProgram: SystemProgram.programId
        })
        .rpc();

    console.log("Your transaction signature", tx);

    counterState = await program.account.count.fetch(counterPDA);
    let electionState = await program.account.election.fetch(electionPDA);

    console.log("Election State:\n", electionState)
    console.log("Election PDA: ", electionPDA.toString());

    console.log("Counter PDA: ", counterPDA.toString());
    console.log("Counter State:\n ", counterState);
  });
});
