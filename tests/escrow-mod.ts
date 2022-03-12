import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { EscrowMod } from "../target/types/escrow_mod";

describe("escrow-mod", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.EscrowMod as Program<EscrowMod>;

  it("Is initialized!", async () => {
    // Add your test here.

    const [bounty, _nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("bounty")),program.provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    console.log("BOUNTY ACCOUNT",bounty.toString());
    const tx = await program.rpc.lockSol(new anchor.BN(100 * anchor.web3.LAMPORTS_PER_SOL),{
      accounts:{
        authority: program.provider.wallet.publicKey,
        systemProgram:anchor.web3.SystemProgram.programId,
        bountyAccount: bounty
      }
    });
    console.log("Your transaction signature", tx);
  });
});
