// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as svelteStore from "svelte/store";
import { writable as persistentStore } from "svelte-persistent-store/dist/local";

import Big from "big.js";
import type { ContractTransaction } from "ethers";
import type { TransactionReceipt } from "@ethersproject/abstract-provider";

import * as error from "./error";
import { store as walletStore } from "./wallet";
import type { Address, Receivers, ReceiverStatus } from "./funding/pool";

import type { SvelteComponent } from "svelte";
import { Icon } from "ui/DesignSystem";

// The store where all managed transactions are stored.
export const store = persistentStore<Tx[]>("transactions", []);

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
  | AnchorProject
  | ClaimRadicleIdentity
  | CollectFunds
  | CreateOrg
  | Erc20Allowance
  | RegisterEnsName
  | SupportOnboarding
  | TopUp
  | UpdateSupport
  | Withdraw;

interface AnchorProject {
  kind: TxKind.AnchorProject;
}

interface CreateOrg {
  kind: TxKind.CreateOrg;
}

interface RegisterEnsName {
  kind: TxKind.RegisterEnsName;
}

interface ClaimRadicleIdentity {
  kind: TxKind.ClaimRadicleIdentity;
  // The claimed Radicle identity root
  root: string;
}

interface Erc20Allowance {
  kind: TxKind.Erc20Allowance;
}

interface SupportOnboarding {
  kind: TxKind.SupportOnboarding;
  // The amount defined as the initial balance
  topUp: string;
  // The amount to be disbursed weekly to the `receivers`.
  budget: string;
  // The receivers of this support.
  receivers: [Address, ReceiverStatus][];
}

interface TopUp {
  kind: TxKind.TopUp;
  amount: string;
}

interface Withdraw {
  kind: TxKind.Withdraw;
  amount: string;
}

interface CollectFunds {
  kind: TxKind.CollectFunds;
  amount: string;
}

interface UpdateSupport {
  kind: TxKind.UpdateSupport;
  // The amount to be disbursed weekly to the `receivers`.
  amount: string;
  // The changes made to the list of receivers.
  receivers: [Address, ReceiverStatus][];
}

export enum TxKind {
  AnchorProject = "Anchor Project",
  ClaimRadicleIdentity = "Claim Radicle Identity",
  CollectFunds = "Collect Funds",
  CreateOrg = "Create Org",
  Erc20Allowance = "ERC-20 Allowance",
  RegisterEnsName = "Register ENS name",
  SupportOnboarding = "Support Onboarding",
  TopUp = "Top Up",
  UpdateSupport = "Update Support",
  Withdraw = "Withdraw",
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

export function anchorProject(txc: ContractTransaction): Tx {
  return { ...txData(txc), kind: TxKind.AnchorProject };
}

export function createOrg(txc: ContractTransaction): Tx {
  return { ...txData(txc), kind: TxKind.CreateOrg };
}

export function registerEnsName(txc: ContractTransaction): Tx {
  return { ...txData(txc), kind: TxKind.RegisterEnsName };
}

export function claimRadicleIdentity(
  txc: ContractTransaction,
  root: string
): Tx {
  return { ...txData(txc), kind: TxKind.ClaimRadicleIdentity, root };
}

export function erc20Allowance(txc: ContractTransaction): Tx {
  return { ...txData(txc), kind: TxKind.Erc20Allowance };
}

export function supportOnboarding(
  txc: ContractTransaction,
  topUp: Big,
  budget: Big,
  receivers: Receivers
): Tx {
  const meta: SupportOnboarding = {
    kind: TxKind.SupportOnboarding,
    topUp: topUp.toString(),
    budget: budget.toString(),
    receivers: [...receivers.entries()],
  };
  return { ...txData(txc), ...meta };
}

export function collect(txc: ContractTransaction, amount: Big): Tx {
  const meta: CollectFunds = {
    kind: TxKind.CollectFunds,
    amount: amount.toString(),
  };
  return { ...txData(txc), ...meta };
}

export function topUp(txc: ContractTransaction, amount: Big): Tx {
  const meta: TopUp = { kind: TxKind.TopUp, amount: amount.toString() };
  return { ...txData(txc), ...meta };
}

export function withdraw(txc: ContractTransaction, amount: Big): Tx {
  const meta: Withdraw = { kind: TxKind.Withdraw, amount: amount.toString() };
  return { ...txData(txc), ...meta };
}

export function updateSupport(
  txc: ContractTransaction,
  amount: Big,
  receivers: Receivers
): Tx {
  const meta: UpdateSupport = {
    kind: TxKind.UpdateSupport,
    amount: amount.toString(),
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

function registerTxUpdateCallback(tx: Tx): void {
  if (tx.status === TxStatus.AwaitingInclusion) {
    const provider = svelteStore.get(walletStore).provider;
    provider
      .waitForTransaction(tx.hash)
      .then(txReceipt => {
        store.update((txs: Tx[]) => {
          const tx_ = txs.find(tx_ => tx_.hash === tx.hash);
          if (tx_) {
            tx_.status = status(txReceipt);
          }
          return txs;
        });
      })
      .catch(err => {
        error.show(
          new error.Error({
            message: "Failed to update transaction status",
            source: err,
          })
        );
      });
  }
}

export function add(tx: Tx): void {
  store.update((txs: Tx[]) => {
    txs.unshift(tx);
    return txs;
  });
  registerTxUpdateCallback(tx);
  cap();
}

// Cap the amount of managed transactions
function cap(length = 7) {
  store.update((txs: Tx[]) => {
    txs.length = Math.min(txs.length, length);
    return txs;
  });
}

async function updateStatuses() {
  const txs = svelteStore.get(store);
  txs.forEach(tx => {
    registerTxUpdateCallback(tx);
  });
}

function status(receipt: TransactionReceipt): TxStatus {
  if (receipt.blockNumber === null || receipt.blockNumber === 0) {
    return TxStatus.AwaitingInclusion;
  } else if (receipt.status === 1) {
    return TxStatus.Included;
  } else {
    return TxStatus.Rejected;
  }
}

/* UI helper functions */

// Middleware criterion to check for an ongoing transaction of a given TxKind.
export function ongoing(txKind: TxKind): (tx: Tx) => boolean {
  return (tx: Tx) =>
    tx.status === TxStatus.AwaitingInclusion && tx.kind === txKind;
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

    case TxKind.AnchorProject:
    case TxKind.CreateOrg:
    case TxKind.ClaimRadicleIdentity:
    case TxKind.Erc20Allowance:
    case TxKind.RegisterEnsName:
    case TxKind.SupportOnboarding:
    case TxKind.TopUp:
    case TxKind.UpdateSupport:
      return Direction.Outgoing;
  }
}

export function emoji(tx: Tx): string {
  switch (tx.kind) {
    case TxKind.AnchorProject:
      return "🏖️";
    case TxKind.CreateOrg:
      return "🎪";
    case TxKind.ClaimRadicleIdentity:
      return "🧦";
    case TxKind.RegisterEnsName:
      return "📇";
    case TxKind.CollectFunds:
    case TxKind.Withdraw:
    case TxKind.Erc20Allowance:
    case TxKind.SupportOnboarding:
    case TxKind.TopUp:
    case TxKind.UpdateSupport:
      return "👛";
  }
}

export function txIcon(tx: Tx): typeof SvelteComponent {
  switch (tx.kind) {
    case TxKind.ClaimRadicleIdentity:
      return Icon.Registered;
    case TxKind.CollectFunds:
    case TxKind.Withdraw:
      return Icon.Withdraw;
    case TxKind.Erc20Allowance:
    case TxKind.RegisterEnsName:
      return Icon.Ethereum;
    case TxKind.SupportOnboarding:
    case TxKind.UpdateSupport:
      return Icon.TokenStreams;
    case TxKind.TopUp:
      return Icon.Topup;
    case TxKind.CreateOrg:
      return Icon.Orgs;
    case TxKind.AnchorProject:
      return Icon.Anchor;
  }
}

export function isIncoming(tx: Tx): boolean {
  return direction(tx) === Direction.Incoming;
}

// The amount the `tx` transfers. `undefined` when not applicable.
export function transferAmount(tx: Tx): Big | undefined {
  switch (tx.kind) {
    case TxKind.CollectFunds:
    case TxKind.Withdraw:
    case TxKind.TopUp:
      return Big(tx.amount);
    case TxKind.SupportOnboarding:
      return Big(tx.topUp);
    default:
      return undefined;
  }
}

// Convert a transaction-related `globalThis.Error` to `error.Error`.
export function convertError(e: globalThis.Error, label: string): error.Error {
  let code: error.Code;
  let message: string;

  if (e.message.includes("gas")) {
    code = error.Code.InsufficientGas;
    message = "you seem to have insufficient gas to cover this transaction.";
  } else if (e.message.toLowerCase().includes("rejected request")) {
    code = error.Code.FailedOrRejectedTransaction;
    message =
      "you have rejected this transaction or it has failed for some unkown reason.";
  } else {
    code = error.Code.UnkownTransactionFailure;
    message = "an unkown transaction error occurred";
  }

  return new error.Error({
    code,
    message: `${label}: ${message}`,
    source: error.fromJsError(e),
  });
}

export function initialize(): void {
  updateStatuses();
}
