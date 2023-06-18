import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { CommitReveal } from "../target/types/commit_reveal";
import { expect } from "chai";

describe("commit-reveal", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CommitReveal as Program<CommitReveal>;
  const provider = program.provider as anchor.AnchorProvider;

  it("initialize!", async () => {
    // Add your test here.
    const signer = provider.wallet;

    const [counterPDA, ] = PublicKey
      .findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("counter")
        ],
        program.programId
      );

    

    const tx = await program.methods.initialize()
        .accounts({
          counter: counterPDA,
          payer: signer.publicKey,
          systemProgram: SystemProgram.programId
        })
        .rpc();

    console.log("Your transaction signature", tx);

    let counterState = await program.account.count.fetch(counterPDA);
    console.log("Counter PDA: ", counterPDA.toString());

    console.log("Counter State\n: ", counterState);
  });
});
