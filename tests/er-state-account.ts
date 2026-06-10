import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { GetCommitmentSignature } from "@magicblock-labs/ephemeral-rollups-sdk";
import { ErStateAccount } from "../target/types/er_state_account";
import { bytes } from "@coral-xyz/anchor/dist/cjs/utils";

const ORACLE_QUEUE = new PublicKey("Cuj97ggrhhidhbu39TijNVqE74xvKJ69gDervRUXAxGh");
const ER_ORACLE_QUEUE = new PublicKey("5hBR571xnXppuCPveTrctfTU7tJLSN94nq7kv7FRK5Tc");


describe("er-state-account", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const providerEphemeralRollup = new anchor.AnchorProvider(
    new anchor.web3.Connection(process.env.EPHEMERAL_PROVIDER_ENDPOINT || "https://devnet.magicblock.app/", {wsEndpoint: process.env.EPHEMERAL_WS_ENDPOINT || "wss://devnet.magicblock.app/"}
    ),
    anchor.Wallet.local()
  );
  console.log("Base Layer Connection: ", provider.connection.rpcEndpoint);
  console.log("Ephemeral Rollup Connection: ", providerEphemeralRollup.connection.rpcEndpoint);
  console.log(`Current SOL Public Key: ${anchor.Wallet.local().publicKey}`)

  before(async function () {
    const balance = await provider.connection.getBalance(anchor.Wallet.local().publicKey)
    console.log('Current balance is', balance / LAMPORTS_PER_SOL, ' SOL','\n')
  })

  const program = anchor.workspace.erStateAccount as Program<ErStateAccount>;

  const programER = new anchor.Program(program.idl, providerEphemeralRollup);

  const userAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user"), anchor.Wallet.local().publicKey.toBuffer()],
    program.programId
  )[0];

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accountsPartial({
      user: anchor.Wallet.local().publicKey,
      userAccount: userAccount,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
    console.log("User Account initialized: ", tx);
  });


  const client_seed = 0;

  it("Non ER rand created!", async () => {
    // Add your test here.
    const tx = await program.methods.createRand(client_seed).accountsPartial({
      payer: anchor.Wallet.local().publicKey,
      userAccount: userAccount,
      oracleQueue: ORACLE_QUEUE,
    })
    .rpc();
    console.log("Non ER randdom created: ", tx);

    let update = false;
    let attempts = 0;

    while(!update && attempts < 3) {
      const account = await program.account.userAccount.fetch(userAccount);
      console.log("Password ", account.password);

      if (account.password.some(bytes => bytes != 0)) {
        console.log("VRF Randomness succeful");
        console.log("VRF Randomness succeful ", account.password);
        update = true;
      } else {
        console.log("Still waiting")
        await new Promise(resolve => setTimeout(resolve, 2000));
        attempts++;
      }
    }

    if (!update) {
      throw new Error("VRF callback failed")
    }

  });

  /*
  it("Update State!", async () => {
    const tx = await program.methods.update(new anchor.BN(42)).accountsPartial({
      user: anchor.Wallet.local().publicKey,
      userAccount: userAccount,
    })
    .rpc();
    console.log("\nUser Account State Updated: ", tx);
  });
  */

  it("Delegate to Ephemeral Rollup!", async () => {

    let tx = await program.methods.delegate().accountsPartial({
      user: anchor.Wallet.local().publicKey,
      userAccount: userAccount,
      validator: new PublicKey("MAS1Dt9qreoRMQ14YQuhg8UTZMMzDdKhmkZMECCzk57"),
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();

    console.log("\nUser Account Delegated to Ephemeral Rollup: ", tx);
  });




  it("ER rand created!", async () => {
    // Add your test here.
    const tx = await programER.methods.createErRand(client_seed).accountsPartial({
      payer: providerEphemeralRollup.wallet.publicKey,
      userAccount: userAccount,
      oracleQueue: ER_ORACLE_QUEUE,
    })
    .rpc({ skipPreflight: true });
    console.log("ER randdom created: ", tx);

  });



  /*
  it("Update State and Commit to Base Layer!", async () => {
    let tx = await program.methods.updateCommit(new anchor.BN(43)).accountsPartial({
      user: providerEphemeralRollup.wallet.publicKey,
      userAccount: userAccount,
    })
    .transaction();

    tx.feePayer = providerEphemeralRollup.wallet.publicKey;

    tx.recentBlockhash = (await providerEphemeralRollup.connection.getLatestBlockhash()).blockhash;
    tx = await providerEphemeralRollup.wallet.signTransaction(tx);
    const txHash = await providerEphemeralRollup.sendAndConfirm(tx, [], {skipPreflight: false});
    const txCommitSgn = await GetCommitmentSignature(
      txHash,
      providerEphemeralRollup.connection
  );

    console.log("\nUser Account State Updated: ", txHash);
  });
  */



  it("Commit and undelegate from Ephemeral Rollup!", async () => {
    let info = await providerEphemeralRollup.connection.getAccountInfo(userAccount);

    console.log("User Account Info: ", info);

    console.log("User account", userAccount.toBase58());

    let tx = await program.methods.undelegate().accounts({
      user: providerEphemeralRollup.wallet.publicKey,
    })
    .transaction();

    tx.feePayer = providerEphemeralRollup.wallet.publicKey;

    tx.recentBlockhash = (await providerEphemeralRollup.connection.getLatestBlockhash()).blockhash;
    tx = await providerEphemeralRollup.wallet.signTransaction(tx);
    const txHash = await providerEphemeralRollup.sendAndConfirm(tx, [], {skipPreflight: false});
    const txCommitSgn = await GetCommitmentSignature(
      txHash,
      providerEphemeralRollup.connection
  );

    console.log("\nUser Account Undelegated: ", txHash);


    let update = false;
    let attempts = 0;

    while(!update && attempts < 3) {
      const account = await program.account.userAccount.fetch(userAccount);
      console.log("Password ", account.password);

      if (account.password.some(bytes => bytes != 0)) {
        console.log("ER VRF Randomness succeful ", account.password);
        update = true;
      } else {
        console.log("Still waiting")
        await new Promise(resolve => setTimeout(resolve, 2000));
        attempts++;
      }

    }

    if (!update) {
      throw new Error("VRF callback failed")
    }

    
  });

  /*
  it("Update State!", async () => {
    let tx = await program.methods.update(new anchor.BN(45)).accountsPartial({
      user: anchor.Wallet.local().publicKey,
      userAccount: userAccount,
    })
    .rpc();

    console.log("\nUser Account State Updated: ", tx);
  });
  */

  it("Close Account!", async () => {
    const tx = await program.methods.close().accountsPartial({
      user: anchor.Wallet.local().publicKey,
      userAccount: userAccount,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
    console.log("\nUser Account Closed: ", tx);
  });
});
