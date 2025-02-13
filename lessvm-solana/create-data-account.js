const {
    Connection,
    Keypair,
    Transaction,
    TransactionInstruction,
    sendAndConfirmTransaction,
    PublicKey,
  } = require('@solana/web3.js');
  const fs = require('fs');
  const path = require('path');
  
  // --- Configuration: Adjust these values ---
  const NETWORK_ENDPOINT = process.env.NETWORK_ENDPOINT || "https://api.devnet.solana.com";
  // Path to your payer keypair (payer must have sufficient SOL)
  const PAYER_KEYPAIR_PATH = process.env.PAYER_KEYPAIR_PATH || path.join(__dirname, "payer.json");
  // The public key of the data account to update (created and funded already)
  const DATA_ACCOUNT_PUBKEY = process.env.DATA_ACCOUNT_PUBKEY || "YourDataAccountPublicKey";
  // The public key of your deployed runtime program (which owns the data account)
  const PROGRAM_ID = process.env.PROGRAM_ID || "YourDeployedProgramPublicKey";
  
  // Utility function to load a keypair from a JSON file
  function loadKeypair(filePath) {
    const keypairData = JSON.parse(fs.readFileSync(filePath, 'utf8'));
    return Keypair.fromSecretKey(new Uint8Array(keypairData));
  }
  
  (async () => {
    try {
      // Connect to the network
      const connection = new Connection(NETWORK_ENDPOINT, "confirmed");
      console.log("Connected to:", NETWORK_ENDPOINT);
  
      // Load the payer keypair
      const payer = loadKeypair(PAYER_KEYPAIR_PATH);
      console.log("Payer public key:", payer.publicKey.toBase58());
  
      // Create PublicKey objects for the data account and your program
      const dataAccountPubkey = new PublicKey(DATA_ACCOUNT_PUBKEY);
      const programId = new PublicKey(PROGRAM_ID);
  
      // Prepare the string you want to store on-chain
      const myString = "Hello, Solana!";  // your string here
      const stringBuffer = Buffer.from(myString, 'utf-8');
      console.log("String to write:", myString);
      console.log("Buffer length:", stringBuffer.length);
  
      // Optionally, you could prepend an opcode or any other header your program expects.
      // For example, if your program expects a 0x01 as a write command:
      const opcode = Buffer.from([0x01]);
      const instructionData = Buffer.concat([opcode, stringBuffer]);
  
      // Build the transaction instruction
      const instruction = new TransactionInstruction({
        keys: [
          { pubkey: dataAccountPubkey, isSigner: false, isWritable: true },
        ],
        programId,  // This should be your deployed runtime program that handles the write
        data: instructionData,
      });
  
      // Create and send the transaction
      const transaction = new Transaction().add(instruction);
      console.log("Sending transaction to write string data into the account...");
  
      const txSignature = await sendAndConfirmTransaction(connection, transaction, [payer]);
      console.log("Transaction successful! Signature:", txSignature);
    } catch (error) {
      console.error("Error writing string to on-chain account:", error);
    }
  })();
  