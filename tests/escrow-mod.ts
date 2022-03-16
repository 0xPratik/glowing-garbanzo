import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { EscrowMod } from "../target/types/escrow_mod";
import {TOKEN_PROGRAM_ID,mintTo,transfer,createMint,getOrCreateAssociatedTokenAccount,mintToChecked,getMint, MINT_SIZE, createInitializeMintInstruction, getAssociatedTokenAddress, createAssociatedTokenAccountInstruction, getAccount,createMintToCheckedInstruction} from "@solana/spl-token"


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
  it("Should read the Data of Bounty Account",async() =>{
    const data = await program.account.bountyAccount.all();
    console.log("Bounty Account Data",data);
  })
  it("Should be able to Claim the Bounty",async() => {
    const [bounty, _nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("bounty")),program.provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    const re = anchor.web3.Keypair.generate()
    console.log("RE",re.publicKey.toString());
    const tx = await program.rpc.claimBounty({
      accounts:{
        authority: program.provider.wallet.publicKey,
        bountyAccount:bounty,
        recieverAccount: re.publicKey,
        systemProgram:anchor.web3.SystemProgram.programId
      },
    })

    console.log("TX Done",tx);
  })
  it("SHould be able to transfer SPL",async() => {

    const lamports = await program.provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);
    const mint = anchor.web3.Keypair.generate();

    const winner_account = anchor.web3.Keypair.generate();

    let winner_ata = await getAssociatedTokenAddress(
      mint.publicKey, // mint
      winner_account.publicKey
    )

    let ata = await getAssociatedTokenAddress(
      mint.publicKey, // mint
      program.provider.wallet.publicKey // owner
    );
    console.log(`ATA: ${ata.toBase58()}`);

    let tx = new anchor.web3.Transaction().add(
      // create mint account
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: program.provider.wallet.publicKey,
        newAccountPubkey: mint.publicKey,
        space: MINT_SIZE,
        lamports: lamports,
        programId: TOKEN_PROGRAM_ID,
      }),
      // init mint account
      createInitializeMintInstruction(
        mint.publicKey, // mint pubkey
        8, // decimals
        program.provider.wallet.publicKey, // mint authority
        program.provider.wallet.publicKey, // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
      ),
      createAssociatedTokenAccountInstruction(
        program.provider.wallet.publicKey,
        ata,
        program.provider.wallet.publicKey,
        mint.publicKey
      ),
      createAssociatedTokenAccountInstruction(
        program.provider.wallet.publicKey,
        winner_ata,
        winner_account.publicKey,
        mint.publicKey
      )
    );

    const res = await program.provider.send(tx,[mint]);

    let new_tx = new anchor.web3.Transaction().add(
      createMintToCheckedInstruction(
        mint.publicKey, // mint
        ata, // receiver (sholud be a token account)
        program.provider.wallet.publicKey, // mint authority
        10000e8, // amount. if your decimals is 8, you mint 10^8 for 1 token.
        8 // decimals
      )
    )

    const res_mint = await program.provider.send(new_tx);

    console.log("MINT TX",res_mint);

    let mintAccount = await getMint(program.provider.connection, mint.publicKey);

    console.log("MINT ACCCOUNT INFO",mintAccount);

    let [vault, vault_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault"),program.provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    const [bounty, _nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("splbounty")),program.provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    console.log("VAULT ACCOUNT",vault.toString());

    const ptx = await program.rpc.lockSpl(new anchor.BN(10000000000),{accounts:{
      authority: program.provider.wallet.publicKey,
      authorityTokenAccount:ata,
      mint:mint.publicKey,
      systemProgram:anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      vaultAccount:vault,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      bountyAccount:bounty
    }})

    console.log("FINAL TX",ptx);

    console.log("WINNER ACCOUNT",winner_ata.toString());

    const withdraw = await program.rpc.unlockSpl({
      accounts:{
        authority: program.provider.wallet.publicKey,
        mint:mint.publicKey,
        systemProgram:anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        vaultAccount:vault,
        winnerTokenAccount:winner_ata,
        bountyAccount:bounty
      }
    })

    console.log("WITH DRAW TX",withdraw);
  })
});



