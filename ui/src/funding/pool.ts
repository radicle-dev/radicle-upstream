// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as svelteStore from "svelte/store";

import * as contract from "./contract";
import * as daiToken from "ui/src/wallet/daiToken";
import * as transaction from "../transaction";
import * as validation from "../validation";

import { Wallet, Status as WalletStatus } from "../wallet";
import * as remote from "../remote";
import { toBaseUnit } from "../ethereum";
import * as error from "ui/src/error";

import Big from "big.js";
import { ContractTransaction, ethers } from "ethers";
import lodash from "lodash";

export const store = svelteStore.writable<Pool | null>(null);

export interface Pool {
  data: remote.Store<PoolData>;

  // Onboard the user's pool with the intial values
  onboard(topUp: Big, weeklyBudget: Big, receivers: Receivers): Promise<void>;

  // Update the weekly budget and the list of receivers.
  updateSettings(weeklyBudget: Big, receivers: Receivers): Promise<void>;

  // Adds funds to the pool. Returns once the transaction has been
  // included in the chain.
  topUp(value: Big): Promise<void>;

  // Withdraw outgoing balance funds to the connected wallet.
  // Returns once the transaction has been included in the chain.
  withdraw(value: Big): Promise<void>;

  // Withdraw all the outgoing balance funds to the connected wallet.
  withdrawAll(): Promise<void>;

  // Collect funds the user has received up to now from givers and
  // transfer them to the users account.
  collect(): Promise<void>;

  // Approve the ERC-20 token allowance, which permits
  // the pool to access said type of token from the user.
  approveErc20(): Promise<void>;
}

// All the data representing a pool.
export interface PoolData {
  // The remaining balance of this pool.
  balance: Big;
  // The weekly amount to be split amongst all the `receivers`.
  weeklyBudget: Big;
  // The list of addresses that receive funds from the pool.
  receivers: Receivers;
  // Funds that the user can collect from their givers.
  collectableFunds: Big;
  // The ERC-20 token allowance. 0 means that the allowance was not
  // granted or that it was fully spent.
  erc20Allowance: Big;
}

/* Receivers */
export type Receivers = Map<Address, ReceiverStatus>;

export type Address = string;
export type Weight = Big;

export enum ReceiverStatus {
  // The receiver is being added
  Added = "Added",
  // The receiver is being removed
  Removed = "Removed",
  // The receiver is present already
  Present = "Present",
}

export function make(wallet: Wallet): Pool {
  const data = remote.createStore<PoolData>();
  const poolAddress = contract.poolAddress(wallet.environment);
  const poolContract = contract.pool(wallet.signer, poolAddress);
  const daiTokenContract = daiToken.connect(wallet.signer, wallet.environment);
  const watcher = new PoolWatcher(poolContract, daiTokenContract);

  loadPoolData(watcher);

  // Periodically refresh the pool data. Particularly useful to
  // reactively display incoming support made available to the user,
  // update the displayed pool remaining balance, etc.
  const POLL_INTERVAL_MILLIS = 1000;
  setInterval(() => {
    loadPoolData(watcher);
  }, POLL_INTERVAL_MILLIS);

  async function loadPoolData(watcher: PoolWatcher) {
    const storedWallet = svelteStore.get(wallet);
    if (storedWallet.status !== WalletStatus.Connected) {
      return;
    }
    const ethAddr = storedWallet.connected.address;
    try {
      data.success(await watcher.poolData(ethAddr));
    } catch (err: unknown) {
      data.error(error.fromUnknown(err));
    }
  }

  async function onboard(
    topUp: Big,
    weeklyBudget: Big,
    receivers: Receivers
  ): Promise<void> {
    return poolContract
      .onboard(topUp, weeklyBudget, toReceiverWeights(receivers))
      .then((tx: ContractTransaction) => {
        transaction.add(
          transaction.supportOnboarding(tx, topUp, weeklyBudget, receivers)
        );
      })
      .finally(() => loadPoolData(watcher));
  }

  async function updateSettings(
    weeklyBudget: Big,
    receivers: Receivers
  ): Promise<void> {
    return poolContract
      .updatePlan(weeklyBudget, toReceiverWeights(receivers))
      .then((tx: ContractTransaction) => {
        const currentReceivers = data.unwrap()?.receivers || new Map();
        const newReceivers = newSetOfReceivers(currentReceivers, receivers);
        transaction.add(
          transaction.updateSupport(tx, weeklyBudget, newReceivers)
        );
      })
      .finally(() => loadPoolData(watcher));
  }

  async function topUp(amount: Big): Promise<void> {
    return poolContract
      .topUp(amount)
      .then((tx: ContractTransaction) => {
        transaction.add(transaction.topUp(tx, amount));
      })
      .finally(() => loadPoolData(watcher));
  }

  async function withdraw(amount: Big): Promise<void> {
    return poolContract
      .withdraw(amount)
      .then(async (tx: ContractTransaction) => {
        transaction.add(transaction.withdraw(tx, amount));
      })
      .finally(() => loadPoolData(watcher));
  }

  async function withdrawAll(): Promise<void> {
    return poolContract
      .withdrawAll()
      .then(async (tx: ContractTransaction) => {
        const remainingBalance = data.unwrap()?.balance || Big(0);
        transaction.add(transaction.withdraw(tx, remainingBalance));
      })
      .finally(() => loadPoolData(watcher));
  }

  async function collect(): Promise<void> {
    return poolContract
      .collect()
      .then((tx: ContractTransaction) => {
        const infoAmount = data.unwrap()?.collectableFunds || Big(0);
        transaction.add(transaction.collect(tx, infoAmount));
      })
      .finally(() => loadPoolData(watcher));
  }

  async function approveErc20(): Promise<void> {
    const unlimited = ethers.BigNumber.from(1).shl(256).sub(1);
    return daiTokenContract
      .approve(poolAddress, unlimited)
      .then((tx: ContractTransaction) => {
        transaction.add(transaction.erc20Allowance(tx));
      })
      .finally(() => loadPoolData(watcher));
  }

  return {
    data,
    onboard,
    updateSettings,
    topUp,
    withdraw,
    withdrawAll,
    collect,
    approveErc20,
  };
}

/**
 * Stores
 */
export const amountStore = svelteStore.writable("");
export const budgetStore = svelteStore.writable("");
export const receiverStore = svelteStore.writable("");

/**
 *
 * Validations
 *
 */

const constraints = {
  // The constraints for a valid weekly budget.
  weeklyBudget: {
    presence: {
      message: "The amount is required",
      allowEmpty: false,
    },
    numericality: {
      strict: false,
      greaterThan: -1,
    },
  },

  // The constraints for a valid pool top up amount input.
  topUpAmount: {
    presence: {
      message: "The amount is required",
      allowEmpty: false,
    },
    numericality: {
      strict: false,
      greaterThan: 0,
    },
  },
};

export const receiverValidationStore = (): validation.ValidationStore => {
  return validation.createValidationStore({}, [
    {
      promise: isAddress,
      validationMessage: "Please provide a valid Ethereum address",
    },
  ]);
};

function isAddress(value: string): Promise<boolean> {
  return Promise.resolve(ethers.utils.isAddress(value));
}

export const weeklyBudgetValidationStore = (): validation.ValidationStore => {
  return validation.createValidationStore(constraints.weeklyBudget);
};

// Validate a balance operation, either a 'Top Up' or a 'Cash out'.
// The provided `balance` represents the account balance upon which
// the value being validated will be compared for sufficiency.
export const balanceValidationStore = (
  balance: Big
): validation.ValidationStore => {
  return validation.createValidationStore(constraints.topUpAmount, [
    {
      promise: amount =>
        Promise.resolve(isValidBig(amount) && balance.gte(Big(amount))),
      validationMessage: "Insufficient balance",
    },
  ]);
};

// Check whether the user has onboarded their pool.
export function isOnboarded(data: PoolData): boolean {
  return (
    data.erc20Allowance.gt(0) &&
    (data.receivers.size > 0 || !data.weeklyBudget.eq(0) || data.balance.gt(0))
  );
}

// Convert `Receivers` to `ReceiverWeight[]`, the latter being the
// representation receivers have in the Radicle Contracts.
function toReceiverWeights(receivers: Receivers): contract.PoolReceiver[] {
  return [...receivers].map(([address, status]) => {
    return { receiver: address, weight: weightForStatus(status) };
  });
}

function weightForStatus(status: ReceiverStatus): number {
  return status === ReceiverStatus.Removed ? 0 : 1;
}

function newSetOfReceivers(current: Receivers, changes: Receivers): Receivers {
  const merged = new Map([...current, ...changes]);
  return new Map(
    [...merged].filter(
      ([_address, status]) => status !== ReceiverStatus.Removed
    )
  );
}

export function isValidBig(value: string): boolean {
  try {
    Big(value);
    return true;
  } catch {
    return false;
  }
}

class PoolWatcher {
  private pool: contract.PoolContract;
  private token: daiToken.ERC20;
  private ethAddr: string | undefined;
  private unwatch: () => void;
  private data: PoolData;
  private getBalance: (now: Date) => Big;
  private getCollectable: (now: Date) => Big;

  constructor(pool: contract.PoolContract, token: daiToken.ERC20) {
    this.pool = pool;
    this.token = token;
    this.ethAddr = undefined;
    this.unwatch = () => void 0;
    this.data = {
      balance: new Big(0),
      weeklyBudget: new Big(0),
      receivers: new Map(),
      collectableFunds: new Big(0),
      erc20Allowance: new Big(0),
    };
    this.getBalance = () => Big(0);
    this.getCollectable = () => Big(0);
  }

  async poolData(ethAddr: string): Promise<PoolData> {
    if (ethAddr !== this.ethAddr) {
      this.ethAddr = ethAddr;
      this.unwatch();

      const unwatchTokenAllowances = await daiToken.watchDaiTokenAllowance(
        this.token,
        ethAddr, // owner
        this.pool.contractAddr(), // spender
        allowance => (this.data.erc20Allowance = allowance)
      );

      const unwatchPoolSender = await this.pool.watchPoolSender(
        ethAddr,
        ({ getBalance, weeklyBudget, receivers }) => {
          this.getBalance = getBalance;
          this.data.weeklyBudget = weeklyBudget;
          this.data.receivers = new Map(
            receivers.map(addr => [addr, ReceiverStatus.Present])
          );
        }
      );

      const unwatchPoolReceiver = await this.pool.watchPoolReceiver(
        ethAddr,
        getCollectable => (this.getCollectable = getCollectable)
      );

      this.unwatch = () => {
        unwatchTokenAllowances();
        unwatchPoolSender();
        unwatchPoolReceiver();
      };
    }

    const now = new Date();
    this.data.collectableFunds = toBaseUnit(this.getCollectable(now));
    this.data.balance = toBaseUnit(this.getBalance(now));
    return lodash.cloneDeep(this.data);
  }
}
