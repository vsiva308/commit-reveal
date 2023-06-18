import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { CommitReveal } from "../target/types/commit_reveal";
import keccak from "keccak";
import { expect } from "chai";
import {createSalt, id, pubkey as selection} from "./config";

describe("commit-reveal", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CommitReveal as Program<CommitReveal>;
  const provider = program.provider as anchor.AnchorProvider;

  it("commit!", async () => {
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
            signer.publicKey.toBuffer()
          ],
          program.programId
        )

    let salt = createSalt(12);
    console.log("Salt: ", salt);
    
    let buf = Buffer.concat([
      selection.toBuffer(),
      Buffer.from(salt, "utf-8")
    ])
    let hash = keccak('keccak256').update(buf).digest();

    console.log("Hash: ", hash.toString('hex'));

    let hashArr = Array.from(new Uint8Array(hash));

    const tx = 
        await program.methods.commit(
          id,
          hashArr
        )
        .accounts({
          election: electionPDA,
          record: recordPDA,
          payer: signer.publicKey,
          systemProgram: SystemProgram.programId
        })
        .rpc();

    console.log("Your transaction signature", tx);

    let recordState = await program.account.voterRecord.fetch(recordPDA);

    console.log("Record State:\n", recordState)
    console.log("Record PDA: ", recordPDA.toString());
  });
});
