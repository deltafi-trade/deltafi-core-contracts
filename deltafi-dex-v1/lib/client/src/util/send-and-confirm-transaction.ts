import {
  sendAndConfirmTransaction as realSendAndConfirmTransaction,
  Keypair,
  Connection,
  Transaction,
  TransactionSignature,
} from '@solana/web3.js';

export const sendAndConfirmTransaction = async (
  title: string,
  connection: Connection,
  transaction: Transaction,
  ...signers: Keypair[]
): Promise<TransactionSignature> => {
  /* tslint:disable:no-console */
  console.info(`Sending ${title} transaction`);

  const txSig = await realSendAndConfirmTransaction(connection, transaction, signers, {
    skipPreflight: false,
    commitment: connection.commitment || 'recent',
    preflightCommitment: connection.commitment || 'recent',
  });
  console.info(`TxSig: ${txSig}`);
  return txSig;
};
