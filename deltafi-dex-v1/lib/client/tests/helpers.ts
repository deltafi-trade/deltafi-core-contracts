import { Keypair, PublicKey, Connection } from '@solana/web3.js';

import { AdminInitializeData } from '../src/instructions';
import { initializeConfig } from '../src/transactions';
import { DEFAULT_FEES, DEFAULT_REWARDS } from './constants';

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function newAccountWithLamports(connection: Connection, lamports = 1000000): Promise<Keypair> {
  const account = Keypair.generate();

  try {
    const airdropSignature = await connection.requestAirdrop(account.publicKey, lamports);
    await connection.confirmTransaction(airdropSignature);
    return account;
  } catch (e) {
    // tslint:disable:no-console
    throw new Error(`Airdrop of ${lamports} failed`);
  }
}

export interface TestConfigInfo {
  config: PublicKey;
  admin: Keypair;
  deltafiMint: PublicKey;
  initData: AdminInitializeData;
}

export async function createTestConfigInfo(connection: Connection, payer: Keypair): Promise<TestConfigInfo> {
  const admin = Keypair.generate();
  const initData: AdminInitializeData = {
    fees: DEFAULT_FEES,
    rewards: DEFAULT_REWARDS,
  };

  return { ...(await initializeConfig(connection, payer, admin, initData)), admin, initData };
}
