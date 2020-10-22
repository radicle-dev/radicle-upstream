import * as svelteStore from "svelte/store";
import { writable as persistentStore } from "svelte-persistent-store/dist/local";

import type { ContractTransaction } from "radicle-contracts/contract-bindings/ethers/Pool";
import { BigNumberish } from "ethers";

import { provider } from "./wallet";

// The store where all managed transactions are stored.
export const store = persistentStore<Tx[]>("radicle-transactions-store", []);

// Periodically refresh the status of all stored transactions.
const POLL_INTERVAL_MILLIS = 10000;
setInterval(() => {
  updateAll();
}, POLL_INTERVAL_MILLIS);

export interface Tx {
  // The hash of the transaction
  hash: string;

  // The status of the transaction
  status: TxStatus;

  // The underlying transaction
  inner: PoolTx;
}

export enum TxStatus {
  // The transaction as been approved and is awaiting to be included in a block.
  AwaitingInclusion = "Awaiting inclusion",
  // The transaction as been included in the block. End of its life cycle.
  Included = "Included",
  // The transaction as been rejected.
  Rejected = "Rejected",
}

type PoolTx =
  | TopUp
  | CollectFunds
  | UpdateMonthlyContribution
  | UpdateBeneficiaries;

interface TopUp {
  kind: PoolTxKind.TopUp;
  amount: BigNumberish;
}

interface CollectFunds {
  kind: PoolTxKind.CollectFunds;
  amount: BigNumberish;
}

interface UpdateMonthlyContribution {
  kind: PoolTxKind.UpdateMonthlyContribution;
  // The value the monthly contribution is being set to.
  amount: BigNumberish;
}

interface UpdateMonthlyContribution {
  kind: PoolTxKind.UpdateMonthlyContribution;
  // The value the monthly contribution is being set to.
  amount: BigNumberish;
}

interface UpdateBeneficiaries {
  kind: PoolTxKind.UpdateBeneficiaries;
}

enum PoolTxKind {
  TopUp = "Top Up",
  CollectFunds = "Collect Funds",
  UpdateMonthlyContribution = "Update Monthly Contribution",
  UpdateBeneficiaries = "Update beneficiaries",
}

/* Smart constructors for `Tx` values */
export function amountPerBlock(txc: ContractTransaction): Tx {
  return {
    hash: txc.hash,
    status: txc.blockNumber ? TxStatus.Included : TxStatus.AwaitingInclusion,
    inner: {
      kind: PoolTxKind.UpdateMonthlyContribution,
      amount: txc.value,
    },
  };
}

export function beneficiaries(txc: ContractTransaction): Tx {
  return {
    hash: txc.hash,
    status: txc.blockNumber ? TxStatus.Included : TxStatus.AwaitingInclusion,
    inner: {
      kind: PoolTxKind.UpdateBeneficiaries,
    },
  };
}

export function collect(txc: ContractTransaction): Tx {
  return {
    hash: txc.hash,
    status: txc.blockNumber ? TxStatus.Included : TxStatus.AwaitingInclusion,
    inner: {
      kind: PoolTxKind.CollectFunds,
      amount: txc.value,
    },
  };
}

export function topUp(txc: ContractTransaction): Tx {
  return {
    hash: txc.hash,
    status: txc.blockNumber ? TxStatus.Included : TxStatus.AwaitingInclusion,
    inner: {
      kind: PoolTxKind.TopUp,
      amount: txc.value,
    },
  };
}

export function add(tx: Tx) {
  store.update(txs => {
    txs.push(tx);
    return txs;
  });
  cap();
}

export function updateStatus(hash: string, status: TxStatus) {
  store.subscribe(txs => {
    const tx = txs.find(tx => tx.hash === hash);
    if (tx) {
      tx.status = status;
    }
  });
}

// Cap the amount of managed transactions
function cap() {
  store.update(txs => {
    txs.length = Math.min(txs.length, 7);
    return txs;
  });
}

async function updateAll() {
  store.update(txs => {
    txs.forEach(async tx => {
      const newStatus = await lookupStatus(tx.hash);
      if (newStatus) tx.status = newStatus;
    });
    return txs;
  });
}

// TODO(nuno): Lookup the actual status of a transaction with the given hash.
async function lookupStatus(hash: string): Promise<TxStatus | undefined> {
  const tx_receipt = await provider.getTransactionReceipt(hash);

  // TODO(nuno): Need to workout a way of detecting failed txs prior to the
  // byzantium fork. Or might not be necessary at all.
  if (tx_receipt.byzantium && tx_receipt.status === 0) {
    return TxStatus.Rejected;
  } else if (tx_receipt.blockNumber === null || tx_receipt.blockNumber === 0) {
    return TxStatus.AwaitingInclusion;
  } else {
    return TxStatus.Included;
  }
}

/* UI helper functions */

export const progressPercentage = (status: TxStatus): number => {
  switch (status) {
    case TxStatus.AwaitingInclusion:
      return 17;
    case TxStatus.Included:
      return 100;
    default:
      return 0;
  }
};

export const colorForStatus = (status: TxStatus): string => {
  switch (status) {
    case TxStatus.AwaitingInclusion:
      return "var(--color-caution)";
    case TxStatus.Rejected:
      return "var(--color-negative)";
    case TxStatus.Included:
      return "var(--color-positive)";
  }
};
