import {
    Connection,
    Keypair,
    Transaction,
    TransactionInstruction,
    sendAndConfirmTransaction,
    PublicKey
  } from '@solana/web3.js';
  import * as fs from 'fs';
  import * as path from 'path';
  
  // --- Configuration: adjust these values ---
  const NETWORK_ENDPOINT = process.env.NETWORK_ENDPOINT || "https://api.devnet.solana.com";
  const PAYER_KEYPAIR_PATH = process.env.PAYER_KEYPAIR_PATH || path.join(__dirname, "payer.json");
  const PROGRAM_ID = process.env.PROGRAM_ID || "YourDeployedProgramPublicKey"; // your runtime program that will process the instruction
  // File or hardcoded string for lessVM bytecode (as a human-readable string)
  const BYTECODE_STRING = fs.existsSync("lessvm_bytecode.txt")
    ? fs.readFileSync("lessvm_bytecode.txt", "utf8").trim()
    : "01 0A 01 05 01 02 14 43 FF"; // example: MulDiv program
  
  // Utility: load a keypair from a JSON file (expects an array of numbers)
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
      console.log("Using bytecode (as string):", BYTECODE_STRING);
  
      // Convert the bytecode string to a buffer (remove spaces if needed)
      const cleanBytecode = BYTECODE_STRING.replace(/\s+/g, "");
      const instructionData = Buffer.from(cleanBytecode, 'hex'); // sending as binary payload
  
      // Build the transaction instruction with the lessVM bytecode in the data field.
      const instruction = new TransactionInstruction({
        keys: [
          // list any additional accounts required by your runtime here
        ],
        programId, 
        data: instructionData,
      });
  
      const tx = new Transaction().add(instruction);
      console.log("Sending transaction with bytecode as parameter...");
  
      const signature = await sendAndConfirmTransaction(connection, tx, [payer]);
      console.log("Transaction successful! Signature:", signature);
    } catch (err) {
      console.error("Error sending transaction:", err);
    }
  })();
  