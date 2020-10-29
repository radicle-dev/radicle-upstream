import * as svelteStore from "svelte/store";
import { writable as persistentStore } from "svelte-persistent-store/dist/local";

import { BigNumberish, ContractTransaction } from "ethers";

import { provider } from "./wallet";

// The store where all managed transactions are stored.
export const store = persistentStore<Tx[]>("radicle-transactions-store", []);

// Periodically refresh the status of all stored transactions.
const POLL_INTERVAL_MILLIS = 10000;
setInterval(() => {
  updateStatuses();
}, POLL_INTERVAL_MILLIS);

export interface Tx {
  // The hash of the transaction
  hash: string;

  // The status of the transaction
  status: TxStatus;

  // The underlying transaction
  inner: PoolTx;

  // The date in which this transaction was created.
  // In milliseconds since epoch.
  date: number;
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
  kind: TxKind.TopUp;
  amount: BigNumberish;
}

interface CollectFunds {
  kind: TxKind.CollectFunds;
  amount: BigNumberish;
}

interface UpdateMonthlyContribution {
  kind: TxKind.UpdateMonthlyContribution;
  // The value the monthly contribution is being set to.
  amount: BigNumberish;
}

interface UpdateBeneficiaries {
  kind: TxKind.UpdateBeneficiaries;
}

export enum TxKind {
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
      kind: TxKind.UpdateMonthlyContribution,
      amount: txc.value.toString(),
    },
    date: Date.now(),
  };
}

export function beneficiaries(txc: ContractTransaction): Tx {
  return {
    hash: txc.hash,
    status: txc.blockNumber ? TxStatus.Included : TxStatus.AwaitingInclusion,
    inner: {
      kind: TxKind.UpdateBeneficiaries,
    },
    date: Date.now(),
  };
}

export function collect(txc: ContractTransaction): Tx {
  return {
    hash: txc.hash,
    status: txc.blockNumber ? TxStatus.Included : TxStatus.AwaitingInclusion,
    inner: {
      kind: TxKind.CollectFunds,
      amount: txc.value.toString(),
    },
    date: Date.now(),
  };
}

export function topUp(txc: ContractTransaction): Tx {
  return {
    hash: txc.hash,
    status: txc.blockNumber ? TxStatus.Included : TxStatus.AwaitingInclusion,
    inner: {
      kind: TxKind.TopUp,
      amount: txc.value.toString(),
    },
    date: Date.now(),
  };
}

export function add(tx: Tx, date = Date.now()) {
  store.update(txs => {
    txs.unshift(tx);
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
function cap(length = 7) {
  store.update(txs => {
    txs.length = Math.min(txs.length, length);
    return txs;
  });
}

async function updateStatuses() {
  store.update(txs => {
    txs
      .filter(tx => tx.status === TxStatus.AwaitingInclusion)
      .forEach(async tx => {
        const newStatus = await lookupStatus(tx.hash);
        if (newStatus) tx.status = newStatus;
      });
    return txs;
  });
}

async function lookupStatus(hash: string): Promise<TxStatus | undefined> {
  const tx_receipt = await provider.getTransactionReceipt(hash);

  console.log(tx_receipt);

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

// Check if there is an ongoing transaction of a given kind.
export function ongoing(txKind: TxKind): boolean {
  const txs: Tx[] = svelteStore.get(store);
  return txs.some(
    tx => tx.status === TxStatus.AwaitingInclusion && tx.inner.kind === txKind
  );
}

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

export interface SummaryCounts {
  rejected: number;
  awaiting: number;
  included: number;

  sum: number;
}

export function summaryCounts(txs: Tx[]): SummaryCounts {
  return txs.reduce(
    (acc, tx): SummaryCounts => {
      acc.sum += 1;
      switch (tx.status) {
        case TxStatus.AwaitingInclusion:
          acc.awaiting += 1;
          break;
        case TxStatus.Rejected:
          acc.rejected += 1;
          break;
        case TxStatus.Included:
          acc.included += 1;
          break;
      }

      return acc;
    },
    {
      rejected: 0,
      awaiting: 0,
      included: 0,
      sum: 0,
    }
  );
}

export const summaryStatus = (counts: SummaryCounts): TxStatus => {
  if (counts.rejected > 0) {
    return TxStatus.Rejected;
  } else if (counts.awaiting > 0) {
    return TxStatus.AwaitingInclusion;
  }

  return TxStatus.Included;
};

export const summaryText = (counts: SummaryCounts): string => {
  let sum = 0;
  let state = "included";

  if (counts.included > 0) {
    sum = counts.included;
  }
  if (counts.rejected > 0) {
    sum = counts.rejected;
    state = "rejected";
  }
  if (counts.awaiting > 0) {
    sum = counts.awaiting;
    state = "awaiting";
  }

  if (sum > 1) {
    return `${sum} transactions ${state}`;
  }

  return `Transaction ${state}`;
};

// A store containing the hash of a transaction selected by the
// user in the TransactionCenter.
export const selectedStore = svelteStore.writable<string>("");
