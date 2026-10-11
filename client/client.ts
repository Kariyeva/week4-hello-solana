const hello = new web3.TransactionInstruction({
  keys: [],
  programId: pg.PROGRAM_ID,
  data: Buffer.from([0]),
});

const echo = new web3.TransactionInstruction({
  keys: [],
  programId: pg.PROGRAM_ID,
  data: Buffer.concat([
    Buffer.from([1]),
    Buffer.from("Привет из Devnet!", "utf8"),
  ]),
});

const transaction = new web3.Transaction().add(hello, echo);

const signature = await web3.sendAndConfirmTransaction(
  pg.connection,
  transaction,
  [pg.wallet.keypair]
);

console.log("Program ID:", pg.PROGRAM_ID.toString());
console.log("Signature:", signature);
console.log(
  "Explorer:",
  `https://explorer.solana.com/tx/${signature}?cluster=devnet`
);
