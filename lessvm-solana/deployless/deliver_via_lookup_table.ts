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
  // Define your lookup table as a JSON object mapping keys to lessVM bytecode strings.
  const lookupTable = {
    "muldiv": "010A010501021443FF",   // example entry for the MulDiv program
    "other": "DEADBEEF"               // add more entries as needed
  };
  // Convert lookup table to a JSON string.
  const lookupTableData = Buffer.from(JSON.stringify(lookupTable), 'utf8');
  // Define the space needed for the lookup table account.
  const LOOKUP_TABLE_SPACE = lookupTableData.length + 100; // add padding if needed
  
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
  
      // Create a new account for the lookup table.
      const lookupTableAccount = Keypair.generate();
      const lamports = await connection.getMinimumBalanceForRentExemption(LOOKUP_TABLE_SPACE);
      console.log("Creating lookup table account:", lookupTableAccount.publicKey.toBase58());
      
      const createAccountTx = new Transaction().add(
        SystemProgram.createAccount({
          fromPubkey: payer.publicKey,
          newAccountPubkey: lookupTableAccount.publicKey,
          lamports,
          space: LOOKUP_TABLE_SPACE,
          programId, // Make the lookup table owned by your runtime program
        })
      );
  
      await sendAndConfirmTransaction(connection, createAccountTx, [payer, lookupTableAccount]);
      console.log("Lookup table account created.");
  
      // Now write the lookup table JSON data into the account.
      // This assumes your runtime program has an instruction to write data into an account.
      // Here we prepend an opcode (say, 0x02) to indicate a lookup table write.
      const opcode = Buffer.from([0x02]);
      const writeData = Buffer.concat([opcode, lookupTableData]);
  
      const writeIx = new TransactionInstruction({
        keys: [
          { pubkey: lookupTableAccount.publicKey, isSigner: false, isWritable: true }
        ],
        programId,
        data: writeData,
      });
  
      const writeTx = new Transaction().add(writeIx);
      console.log("Writing lookup table data into account...");
      await sendAndConfirmTransaction(connection, writeTx, [payer]);
      console.log("Lookup table data written.");
  
      // Finally, send a transaction that uses the lookup table.
      // For example, pass the key "muldiv" as the parameter so that your runtime can look up the associated bytecode.
      const keyToLookup = Buffer.from("muldiv", 'utf8');
      const lookupIx = new TransactionInstruction({
        keys: [
          { pubkey: lookupTableAccount.publicKey, isSigner: false, isWritable: false }
        ],
        programId,
        data: keyToLookup, // your runtime will use this key to fetch the corresponding bytecode from the table
      });
  
      const lookupTx = new Transaction().add(lookupIx);
      console.log("Sending transaction that references the lookup table...");
      const signature = await sendAndConfirmTransaction(connection, lookupTx, [payer]);
      console.log("Transaction successful! Signature:", signature);
    } catch (err) {
      console.error("Error in lookup table delivery:", err);
    }
  })();
  