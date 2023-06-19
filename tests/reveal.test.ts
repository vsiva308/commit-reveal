import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { CommitReveal } from "../target/types/commit_reveal";
import keccak from "keccak";
import { expect } from "chai";
import {getID, getSalt, getPubkey} from "./config";

describe("commit-reveal", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CommitReveal as Program<CommitReveal>;
  const provider = program.provider as anchor.AnchorProvider;

  //get config
  const salt = getSalt();
  const selection = PublicKey.default;
  const id = getID();

  it("reveal!", async () => {
    // Add your test here.
    const signer = provider.wallet;

    const [electionPDA, ] = PublicKey
        .findProgramAddressSync(
            [
                anchor.utils.bytes.utf8.encode("election"),
                id.toArrayLike(Buffer, "be", 8)
            ],
            program.programId
        );

    const [recordPDA, ] = PublicKey
        .findProgramAddressSync(
          [
            anchor.utils.bytes.utf8.encode("record"),
            electionPDA.toBuffer(),
            signer.publicKey.toBuffer()
          ],
          program.programId
        )

    const tx = 
        await program.methods.reveal(
          salt,
          id
        )
        .accounts({
          election: electionPDA,
          record: recordPDA,
          selection: selection,
          payer: signer.publicKey,
        })
        .rpc();

    console.log("Your transaction signature", tx);

    let electionState = await program.account.election.fetch(electionPDA);

    console.log("Record State:\n", electionState)
    console.log("Record PDA: ", electionPDA.toString());
  });
});
