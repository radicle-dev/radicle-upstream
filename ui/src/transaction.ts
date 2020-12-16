import * as svelteStore from "svelte/store";
import { writable as persistentStore } from "svelte-persistent-store/dist/local";

import type { BigNumberish, ContractTransaction } from "ethers";
import type { TransactionReceipt } from "@ethersproject/abstract-provider";

import { provider } from "./wallet";
import type { Address, Receivers, ReceiverStatus } from "./funding/pool";

// The store where all managed transactions are stored.
export const store = persistentStore<Tx[]>("radicle-transactions", []);

// Periodically refresh the status of all ongoing transactions.
const POLL_INTERVAL_MILLIS = 3000;
setInterval(() => {
  updateStatuses();
}, POLL_INTERVAL_MILLIS);

export type Tx = TxData & MetaTx;

// The data shared across all types of transactions
// that we are to deal with.
export interface TxData {
  // The hash of the transaction
  hash: string;

  // The status of the transaction
  status: TxStatus;

  // The date in which this transaction was created.
  // In milliseconds since epoch.
  date: number;

  // The sender of this transaction
  from: string;

  // The destination of this transaction
  to?: string;
}

// The meta transactions that we provide to the user.
type MetaTx =
  | Erc20Allowance
  | SupportOnboarding
  | TopUp
  | CollectFunds
  | UpdateSupport
  | Withdraw;

interface Erc20Allowance {
  kind: TxKind.Erc20Allowance;
}

interface SupportOnboarding {
  kind: TxKind.SupportOnboarding;
  // The amount defined as the initial balance
  topUp: BigNumberish;
  // The amount to be disbursed monthly to the `receivers`.
  budget: BigNumberish;
  // The receivers of this support.
  receivers: [Address, ReceiverStatus][];
}

interface TopUp {
  kind: TxKind.TopUp;
  amount: BigNumberish;
}

interface Withdraw {
  kind: TxKind.Withdraw;
  amount: BigNumberish;
}

interface CollectFunds {
  kind: TxKind.CollectFunds;
  amount: BigNumberish;
}

interface UpdateSupport {
  kind: TxKind.UpdateSupport;
  // The amount to be disbursed monthly to the `receivers`.
  amount: BigNumberish;
  // The changes made to the list of receivers.
  receivers: [Address, ReceiverStatus][];
}

export enum TxKind {
  Erc20Allowance = "ERC-20 Allowance",
  SupportOnboarding = "Support Onboarding",
  Withdraw = "Withdraw",
  TopUp = "Top Up",
  CollectFunds = "Collect Funds",
  UpdateSupport = "Update Support",
}

export enum TxStatus {
  // The transaction as been approved and is awaiting to be included in a block.
  AwaitingInclusion = "Awaiting inclusion",
  // The transaction as been included in the block. End of its life cycle.
  Included = "Included",
  // The transaction as been rejected.
  Rejected = "Rejected",
}

/* Smart constructors for `Tx` values */

export function erc20Allowance(txc: ContractTransaction): Tx {
  return { ...txData(txc), ...{ kind: TxKind.Erc20Allowance } };
}

export function supportOnboarding(
  txc: ContractTransaction,
  topUp: BigNumberish,
  budget: BigNumberish,
  receivers: Receivers
): Tx {
  const meta: SupportOnboarding = {
    kind: TxKind.SupportOnboarding,
    topUp,
    budget,
    receivers: [...receivers.entries()],
  };
  return { ...txData(txc), ...meta };
}

export function collect(txc: ContractTransaction, amount: BigNumberish): Tx {
  const meta: CollectFunds = {
    kind: TxKind.CollectFunds,
    amount,
  };
  return { ...txData(txc), ...meta };
}

export function topUp(txc: ContractTransaction): Tx {
  const meta: TopUp = { kind: TxKind.TopUp, amount: txc.value.toString() };
  return { ...txData(txc), ...meta };
}

export function withdraw(txc: ContractTransaction, amount: BigNumberish): Tx {
  const meta: Withdraw = { kind: TxKind.Withdraw, amount };
  return { ...txData(txc), ...meta };
}

export function updateSupport(
  txc: ContractTransaction,
  amount: BigNumberish,
  receivers: Receivers
): Tx {
  const meta: UpdateSupport = {
    kind: TxKind.UpdateSupport,
    amount,
    receivers: [...receivers.entries()],
  };
  return { ...txData(txc), ...meta };
}

function txData(txc: ContractTransaction, date: number = Date.now()): TxData {
  return {
    hash: txc.hash,
    status: txc.blockNumber ? TxStatus.Included : TxStatus.AwaitingInclusion,
    date,
    from: txc.from,
    to: txc.to,
  };
}

export function add(tx: Tx): void {
  store.update(txs => {
    txs.unshift(tx);
    return txs;
  });
  cap();
}

export function updateStatus(hash: string, status: TxStatus): void {
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
        const receipt = await provider.getTransactionReceipt(tx.hash);
        const newStatus = await status(receipt);
        if (newStatus) tx.status = newStatus;
      });
    return txs;
  });
}

async function status(
  receipt: TransactionReceipt
): Promise<TxStatus | undefined> {
  // TODO(nuno): Need to workout a way of detecting failed txs prior to the
  // byzantium fork. Or might not be necessary at all.
  if (receipt.byzantium && receipt.status === 0) {
    return TxStatus.Rejected;
  } else if (receipt.blockNumber === null || receipt.blockNumber === 0) {
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
    tx => tx.status === TxStatus.AwaitingInclusion && tx.kind === txKind
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

// The direction of a transaction in respect to the local user,
// assuming that they are involved either as the sender or as the
// recipient.
export enum Direction {
  // The local user's linked account is the recipient of the transaction.
  Incoming = "Incoming",
  // The local user's linked account is the sender of the transaction.
  Outgoing = "Outgoing",
}

function direction(tx: Tx): Direction {
  switch (tx.kind) {
    case TxKind.CollectFunds:
    case TxKind.Withdraw:
      return Direction.Incoming;
    case TxKind.Erc20Allowance:
    case TxKind.SupportOnboarding:
    case TxKind.TopUp:
    case TxKind.UpdateSupport:
      return Direction.Outgoing;
  }
}

export function isIncoming(tx: Tx): boolean {
  return direction(tx) === Direction.Incoming;
}

// The amount the `tx` transfers. `undefined` when not applicable.
export function transferAmount(tx: Tx): BigNumberish | undefined {
  switch (tx.kind) {
    case TxKind.CollectFunds:
    case TxKind.Withdraw:
    case TxKind.TopUp:
      return tx.amount;
    case TxKind.SupportOnboarding:
      return tx.topUp;
    default:
      return undefined;
  }
}

export function formatDate(date: Date): string {
  return `${date.getHours()}:${date.getMinutes()}:${date.getSeconds()} on ${date.getUTCDate()} ${
    monthNames[date.getMonth()]
  } ${date.getFullYear()}`;
}

const monthNames = [
  "January",
  "February",
  "March",
  "April",
  "May",
  "June",
  "July",
  "August",
  "September",
  "October",
  "November",
  "December",
];
