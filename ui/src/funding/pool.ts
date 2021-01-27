import * as svelteStore from "svelte/store";

import * as contract from "./contract";
import * as transaction from "../transaction";
import * as validation from "../validation";

import { Wallet, Account, Status as WalletStatus } from "../wallet";
import * as remote from "../remote";
import { BigNumber, BigNumberish, ContractTransaction, ethers } from "ethers";

export const store = svelteStore.writable<Pool | null>(null);

export interface Pool {
  data: remote.Store<PoolData>;

  // Get the account that owns this pool.
  getAccount: () => Account | undefined;

  // Onboard the user's pool with the intial values
  onboard(
    topUp: BigNumber,
    amountPerBlock: BigNumber,
    receivers: Receivers
  ): Promise<void>;

  // Update the contribution per block and the list of receivers.
  updateSettings(
    amountPerBlock: BigNumber,
    receivers: Receivers
  ): Promise<void>;

  // Adds funds to the pool. Returns once the transaction has been
  // included in the chain.
  topUp(value: BigNumber): Promise<void>;

  // Withdraw outgoing balance funds to the connected wallet.
  // Returns once the transaction has been included in the chain.
  withdraw(value: BigNumber): Promise<void>;

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
  balance: BigNumber;
  // The total amount to be disbursed to all receivers with each block.
  amountPerBlock: BigNumber;
  // The list of addresses that receive funds from the pool.
  receivers: Receivers;
  // Funds that the user can collect from their givers.
  collectableFunds: BigNumber;
  // The ERC-20 token allowance. 0 means that the allowance was not
  // granted or that it was fully spent.
  erc20Allowance: BigNumber;
}

/* Receivers */
export type Receivers = Map<Address, ReceiverStatus>;

export type Address = string;
export type Weight = BigNumber;

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
  const poolContract = contract.pool(wallet.signer);
  const daiTokenContract = contract.daiToken(wallet.signer);
  loadPoolData();

  // Periodically refresh the pool data. Particularly useful to
  // reactively display incoming support made available to the user,
  // update the displayed pool remaining balance, etc.
  const POLL_INTERVAL_MILLIS = 10000;
  setInterval(() => {
    loadPoolData();
  }, POLL_INTERVAL_MILLIS);

  async function loadPoolData() {
    if (svelteStore.get(wallet).status !== WalletStatus.Connected) return;

    try {
      const balance = await poolContract.withdrawable();
      const collectableFunds = await poolContract.collectable();
      const amountPerBlock = await poolContract.amountPerBlock();
      const contractReceivers = await poolContract.receivers();
      const receivers = new Map<Address, ReceiverStatus>(
        contractReceivers.map((e: contract.PoolReceiver) => [
          e.receiver,
          ReceiverStatus.Present,
        ])
      );
      const erc20Allowance = await getErc20Allowance();

      data.success({
        // Handle potential overflow using BN.js
        balance,
        amountPerBlock,
        receivers,
        // Handle potential overflow using BN.js
        collectableFunds,
        erc20Allowance,
      });
    } catch (error) {
      data.error(error);
    }
  }

  function getAccount(): Account | undefined {
    const w = svelteStore.get(wallet);
    return w.status === WalletStatus.Connected
      ? w.connected.account
      : undefined;
  }

  async function onboard(
    topUp: BigNumber,
    amountPerBlock: BigNumber,
    receivers: Receivers
  ): Promise<void> {
    return poolContract
      .onboard(topUp, amountPerBlock, toReceiverWeights(receivers))
      .then((tx: ContractTransaction) => {
        transaction.add(
          transaction.supportOnboarding(tx, topUp, amountPerBlock, receivers)
        );
      })
      .finally(loadPoolData);
  }

  async function updateSettings(
    amountPerBlock: BigNumber,
    receivers: Receivers
  ): Promise<void> {
    return poolContract
      .updatePlan(amountPerBlock, toReceiverWeights(receivers))
      .then((tx: ContractTransaction) => {
        const currentReceivers = data.unwrap()?.receivers || new Map();
        const newReceivers = newSetOfReceivers(currentReceivers, receivers);
        transaction.add(
          transaction.updateSupport(tx, amountPerBlock, newReceivers)
        );
      })
      .finally(loadPoolData);
  }

  async function topUp(amount: BigNumber): Promise<void> {
    return poolContract
      .topUp(amount)
      .then((tx: ContractTransaction) => {
        transaction.add(transaction.topUp(tx, amount));
      })
      .finally(loadPoolData);
  }

  async function withdraw(amount: BigNumber): Promise<void> {
    return poolContract
      .withdraw(amount)
      .then(async (tx: ContractTransaction) => {
        const ALL = await poolContract.withdrawAllFlag();
        const infoAmount = amount.eq(ALL)
          ? data.unwrap()?.balance || BigNumber.from(0)
          : amount;
        transaction.add(transaction.withdraw(tx, infoAmount));
      })
      .finally(loadPoolData);
  }

  async function withdrawAll(): Promise<void> {
    const ALL = await poolContract.withdrawAllFlag();
    return withdraw(ALL);
  }

  async function collect(): Promise<void> {
    return poolContract
      .collect()
      .then((tx: ContractTransaction) => {
        const infoAmount = data.unwrap()?.collectableFunds || BigNumber.from(0);
        transaction.add(transaction.collect(tx, infoAmount));
      })
      .finally(loadPoolData);
  }

  async function getErc20Allowance(): Promise<BigNumber> {
    const account = getAccount();
    if (account) {
      return daiTokenContract.allowance(
        account.address,
        svelteStore.get(contract.POOL_ADDRESS)
      );
    } else {
      return BigNumber.from(0);
    }
  }

  async function approveErc20(): Promise<void> {
    const unlimited = BigNumber.from(1).shl(256).sub(1);
    return daiTokenContract
      .approve(svelteStore.get(contract.POOL_ADDRESS), unlimited)
      .then((tx: ContractTransaction) => {
        transaction.add(transaction.erc20Allowance(tx));
      })
      .finally(loadPoolData);
  }

  return {
    data,
    getAccount,
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
  // The constraints for a valid monthly contribution.
  monthlyContribution: {
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

export const monthlyContributionValidationStore = (): validation.ValidationStore => {
  return validation.createValidationStore(constraints.monthlyContribution);
};

// Validate a balance operation, either a 'Top Up' or a 'Cash out'.
// The provided `balance` represents the account balance upon which
// the value being validated will be compared for sufficiency.
export const balanceValidationStore = (
  balance: BigNumberish
): validation.ValidationStore => {
  return validation.createValidationStore(constraints.topUpAmount, [
    {
      promise: amount =>
        Promise.resolve(
          isValidBigNumber(amount) && BigNumber.from(balance).gte(amount)
        ),
      validationMessage: "Insufficient balance",
    },
  ]);
};

// Check whether the user has onboarded their pool.
export function isOnboarded(data: PoolData): boolean {
  return (
    data.erc20Allowance.gt(0) &&
    (data.receivers.size > 0 ||
      !data.amountPerBlock.isZero() ||
      data.balance.gt(0))
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
    [...merged].filter(([_address, status]) => status != ReceiverStatus.Removed)
  );
}

// Check whether the given value can be converted to `ethers.BigNumber`.
// N.B: ethers.BigNumber.isBigNumber doesn't work as expected:
// https://github.com/ethers-io/ethers.js/blob/master/packages/bignumber/src.ts/bignumber.ts#L285
export function isValidBigNumber(value: string): boolean {
  try {
    BigNumber.from(value);
    return true;
  } catch {
    return false;
  }
}
