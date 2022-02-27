import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Paymentchannel } from "../target/types/paymentchannel";

describe("paymentchannel", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  let user1 = anchor.web3.Keypair.generate();
  let user2 = anchor.web3.Keypair.generate();
  let channel_acc = anchor.web3.Keypair.generate();
  const program = anchor.workspace.Paymentchannel as Program<Paymentchannel>;

  it("Is initialized!", async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user1.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL), "confirmed");
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user2.publicKey, 1.5 * anchor.web3.LAMPORTS_PER_SOL), "confirmed");
    
    console.log(await provider.connection.getBalance(user1.publicKey, "processed") / anchor.web3.LAMPORTS_PER_SOL);
    console.log("Initializing")
    await program.transaction.initialize(
      new anchor.BN(0.5*anchor.web3.LAMPORTS_PER_SOL), new anchor.BN(0.8*anchor.web3.LAMPORTS_PER_SOL), new anchor.BN(Date.now()+100000), {
      accounts: {
        channel: channel_acc.publicKey,
        user1: user1.publicKey,
        user2: user2.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers:[user1,user2]
    });
    console.log(await provider.connection.getBalance(user1.publicKey, "processed") / anchor.web3.LAMPORTS_PER_SOL);
  });
});
