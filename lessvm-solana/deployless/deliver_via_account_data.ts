import {
    Connection,
    Keypair,
    Transaction,
    TransactionInstruction,
    sendAndConfirmTransaction,
    PublicKey,
    SystemProgram
  } from '@solana/web3.js';
  import * as fs from 'fs';
  import * as path from 'path';
  
  // --- Configuration ---
  const NETWORK_ENDPOINT = process.env.NETWORK_ENDPOINT || "https://api.devnet.solana.com";
  const PAYER_KEYPAIR_PATH = process.env.PAYER_KEYPAIR_PATH || path.join(__dirname, "payer.json");
  const PROGRAM_ID = process.env.PROGRAM_ID || "YourDeployedProgramPublicKey"; 
  // Path to the binary file containing lessVM bytecode (for example, generated by converting hex to binary)
  const BYTECODE_FILE = process.env.BYTECODE_FILE || "lessvm_program.bin";
  
  // Determine the space needed (for example, the file size plus some padding)
  const bytecodeBuffer = fs.readFileSync(BYTECODE_FILE);
  const ACCOUNT_SPACE = bytecodeBuffer.length + 100; // adjust padding if needed
  
  function loadKeypair(filePath: string): Keypair {
    const keypairData = JSON.parse(fs.readFileSync(filePath, 'utf8'));
    return Keypair.fromSecretKey(new Uint8Array(keypairData));
  }
  
  (async () => {
    try {
      const connection = new Connection(NETWORK_ENDPOINT, "confirmed");
      const payer = loadKeypair(PAYER_KEYPAIR_PATH);
      const programId = new PublicKey(PROGRAM_ID);
  
      console.log("Network:", NETWORK_ENDPOINT);
      console.log("Payer:", payer.publicKey.toBase58());
  
      // Create a new account to hold the bytecode.
      const dataAccount = Keypair.generate();
      const lamports = await connection.getMinimumBalanceForRentExemption(ACCOUNT_SPACE);
      console.log("Creating data account:", dataAccount.publicKey.toBase58());
  
      const createDataAccountTx = new Transaction().add(
        SystemProgram.createAccount({
          fromPubkey: payer.publicKey,
          newAccountPubkey: dataAccount.publicKey,
          lamports,
          space: ACCOUNT_SPACE,
          programId, // Set owner to your runtime program
        })
      );
      await sendAndConfirmTransaction(connection, createDataAccountTx, [payer, dataAccount]);
      console.log("Data account created.");
  
      // Write the binary lessVM bytecode into the account.
      // Prepend an opcode (say, 0x03) if your runtime expects it.
      const opcode = Buffer.from([0x03]);
      const writePayload = Buffer.concat([opcode, bytecodeBuffer]);
  
      const writeIx = new TransactionInstruction({
        keys: [
          { pubkey: dataAccount.publicKey, isSigner: false, isWritable: true }
        ],
        programId,
        data: writePayload,
      });
  
      const writeTx = new Transaction().add(writeIx);
      console.log("Writing bytecode to data account...");
      const signature = await sendAndConfirmTransaction(connection, writeTx, [payer]);
      console.log("Bytecode written. Transaction signature:", signature);
  
      // Now you can invoke your runtime program, passing the data account's public key as an argument.
      // Example invocation:
      const invokeIx = new TransactionInstruction({
        keys: [
          { pubkey: dataAccount.publicKey, isSigner: false, isWritable: true }
        ],
        programId,
        data: Buffer.from([]), // No additional data required if your program knows to read from the account.
      });
  
      const invokeTx = new Transaction().add(invokeIx);
      console.log("Invoking program with data account reference...");
      const invokeSignature = await sendAndConfirmTransaction(connection, invokeTx, [payer]);
      console.log("Program invoked successfully! Signature:", invokeSignature);
    } catch (err) {
      console.error("Error delivering via account data:", err);
    }
  })();
  