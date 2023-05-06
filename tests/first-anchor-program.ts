import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { FirstAnchorProgram } from "../target/types/first_anchor_program";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";

describe("first-anchor-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FirstAnchorProgram as Program<FirstAnchorProgram>;

  it("test GM!", async () => {
    const tx = await program.methods.myGmInstruction()
      .accounts({
        gmProgram: new anchor.web3.PublicKey("GMoMVeaVmD8H2JqgbSaSBuELofJus3z5Lb97AfKFrw3k")
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });



  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const dataAccountKP = anchor.web3.Keypair.generate();
  //   const user = anchor.web3.Keypair.fromSecretKey(new Uint8Array([18,218,75,18,172,230,123,80,217,178,14,245,248,73,25,30,220,31,199,219,52,156,31,172,97,234,71,229,167,17,236,218,7,36,48,193,246,27,187,118,192,207,222,127,70,209,214,46,194,243,201,21,87,77,64,160,151,2,202,28,131,2,171,93]))
  //   const tx = await program.methods.myInstruction(new anchor.BN(666))
  //     .accounts({
  //       dataAccount: dataAccountKP.publicKey,
  //       user: user.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId
  //     })
  //     .signers([user, dataAccountKP])
  //     .rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Test sum!", async () => {
  //   // Add your test here.
  //   const user = anchor.web3.Keypair.fromSecretKey(new Uint8Array([18,218,75,18,172,230,123,80,217,178,14,245,248,73,25,30,220,31,199,219,52,156,31,172,97,234,71,229,167,17,236,218,7,36,48,193,246,27,187,118,192,207,222,127,70,209,214,46,194,243,201,21,87,77,64,160,151,2,202,28,131,2,171,93]))
  //   const tx = await program.methods.mySum()
  //     .accounts({
  //       dataAccount1: new anchor.web3.PublicKey("DKUM39fiEA4WSZFjK68NYEaZkZEkrSn2QjGHXLD9TYCR"),
  //       dataAccount2: new anchor.web3.PublicKey("uB7kHsjNKrFg1NsCFhFTfMwoiZs6Y5GgdigjthUPMEF"),
  //     })
  //     //.signers([user])
  //     .rpc();
  //   console.log("Your transaction signature", tx);
  // });
});
