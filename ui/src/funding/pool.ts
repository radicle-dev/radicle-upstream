import * as svelteStore from "svelte/store";
import { PoolFactory } from "radicle-contracts/build/contract-bindings/ethers/PoolFactory";
import type { Pool as PoolContract } from "radicle-contracts/contract-bindings/ethers/Pool";

import * as transaction from "../transaction";
import * as validation from "../validation";

import { Wallet, Account, Status } from "../wallet";
import * as remote from "../remote";
import type { BigNumberish } from "ethers";

export const store = svelteStore.writable<Pool | null>(null);

export interface Pool {
  data: remote.Store<PoolData>;

  // Get the account that owns this pool. Should be the connected wallet account.
  getAccount: () => Account | undefined;

  // Onboard the user's pool with the intial values
  onboard(
    amountPerBlock: BigNumberish,
    receivers: Receivers,
    initialBalance: BigNumberish
  ): Promise<void>;

  // Update the contribution per block and the list of receivers.
  updateSettings(amountPerBlock: string, receivers: Receivers): Promise<void>;

  // Adds funds to the pool. Returns once the transaction has been
  // included in the chain.
  topUp(value: number): Promise<void>;

  // Withdraw outgoing balance funds to the connected wallet.
  // Returns once the transaction has been included in the chain.
  withdraw(value: number): Promise<void>;

  // Collect funds the user has received up to now from givers and
  // transfer them to the users account.
  collect(): Promise<void>;
}

// The pool settings the user can change and save.
export interface PoolSettings {
  // The total amount to be disbursed to all receivers with each block.
  amountPerBlock: string;
  // The list of addresses that receive funds from the pool.
  receivers: string[];
}

// All the data representing a pool.
export interface PoolData {
  // The remaining balance of this pool.
  balance: number;
  // The total amount to be disbursed to all receivers with each block.
  amountPerBlock: string;
  // The list of addresses that receive funds from the pool.
  receivers: Receivers;
  // Funds that the user can collect from their givers.
  collectableFunds: number;
}

export type Weight = number;
export type Receivers = Map<Address, Weight>;

export const CONTRACT_ADDRESS: string =
  "0x0e22b57c7e69d1b62c9e4c88bb63b0357a905d1e";

export function make(wallet: Wallet): Pool {
  const data = remote.createStore<PoolData>();
  const poolContract: PoolContract = PoolFactory.connect(
    CONTRACT_ADDRESS,
    wallet.signer
  );

  loadPoolData();

  async function loadPoolData() {
    try {
      const balance = await poolContract.withdrawable();
      const collectableFunds = await poolContract.collectable();
      const amountPerBlock = await poolContract.getAmountPerBlock();
      const contract_receivers = await poolContract.getAllReceivers();
      const receivers = new Map(
        contract_receivers.map(e => [e.receiver, e.weight])
      );

      data.success({
        // Handle potential overflow using BN.js
        balance: Number(balance),
        amountPerBlock: amountPerBlock.toString(),
        receivers,
        // Handle potential overflow using BN.js
        collectableFunds: Number(collectableFunds),
      });
    } catch (error) {
      data.error(error);
    }
  }

  function getAccount(): Account | undefined {
    const w = svelteStore.get(wallet);
    return w.status === Status.Connected ? w.connected.account : undefined;
  }

  async function updateAmountPerBlock(amount: BigNumberish): Promise<void> {
    await poolContract
      .setAmountPerBlock(amount)
      .then(tx => {
        transaction.add(transaction.amountPerBlock(tx));
        tx.wait();
      })
      .finally(loadPoolData);
  }

  async function onboard(
    amountPerBlock: string,
    receivers: Receivers,
    initialBalance: number
  ): Promise<void> {
    return updateAmountPerBlock(amountPerBlock)
      .then(_ => updateReceiverAddresses(receivers))
      .then(_ => topUp(initialBalance))
      .finally(loadPoolData);
  }

  async function updateSettings(
    amountPerBlock: string,
    receivers: Receivers
  ): Promise<void> {
    return updateAmountPerBlock(amountPerBlock)
      .then(_ => updateReceiverAddresses(receivers))
      .finally(loadPoolData);
  }

  async function updateReceiverAddresses(receivers: Receivers): Promise<void> {
    const txs = [...receivers.entries()].map(([address, weight]) =>
      poolContract.setReceiver(address, weight).then(tx => {
        transaction.add(transaction.beneficiaries(tx));
        tx.wait();
      })
    );

    await Promise.all(txs).finally(loadPoolData);
  }

  async function topUp(value: BigNumberish): Promise<void> {
    const tx = await poolContract.topUp({
      gasLimit: 200 * 1000,
      value,
    });
    transaction.add(transaction.topUp(tx));
    const receipt = await tx.wait();
    if (receipt.status === 0) {
      throw new Error(`Transaction reverted: ${receipt.transactionHash}`);
    }
    loadPoolData();
  }

  async function withdraw(amount: number): Promise<void> {
    const tx = await poolContract.withdraw(amount);
    transaction.add(transaction.withdraw(tx));
    const receipt = await tx.wait();
    if (receipt.status === 0) {
      throw new Error(`Transaction reverted: ${receipt.transactionHash}`);
    }
    loadPoolData();
  }

  async function collect(): Promise<void> {
    const tx = await poolContract.collect();
    transaction.add(transaction.collect(tx));
    const receipt = await tx.wait();
    if (receipt.status === 0) {
      throw new Error(`Transaction reverted: ${receipt.transactionHash}`);
    }
    loadPoolData();
  }

  return {
    data,
    getAccount,
    onboard,
    updateSettings,
    topUp,
    withdraw,
    collect,
  };
}

/**
 * Stores
 */
export const membersStore = svelteStore.writable("");
export const amountStore = svelteStore.writable("");
export const budgetStore = svelteStore.writable("");

/**
 *
 * Validations
 *
 */

// Patterns
const COMMA_SEPARATED_LIST = /(^[-\w\s]+(?:,[-\w\s]*)*$)?/;

const contraints = {
  // The contraints for a valid input list of members.
  members: {
    format: {
      pattern: COMMA_SEPARATED_LIST,
      message: `Should be a comma-separated list of addresses`,
    },
  },

  // The contraints for a valid monthly contribution.
  monthlyContribution: {
    presence: {
      message: "The amount is required",
      allowEmpty: false,
    },
    numericality: {
      strict: true,
      greaterThan: -1,
    },
  },

  // The contraints for a valid pool top up amount input.
  topUpAmount: {
    presence: {
      message: "The amount is required",
      allowEmpty: false,
    },
    numericality: {
      strict: true,
      greaterThan: 0,
    },
  },
};

export const membersValidationStore: validation.ValidationStore = validation.createValidationStore(
  contraints.members
);

export const monthlyContributionValidationStore = (): validation.ValidationStore => {
  return validation.createValidationStore(contraints.monthlyContribution);
};

export const topUpAmountValidationStore = (
  balance: BigNumberish
): validation.ValidationStore => {
  return validation.createValidationStore(contraints.topUpAmount, [
    {
      promise: amount => Promise.resolve(balance >= amount),
      validationMessage: "Insufficient balance",
    },
  ]);
};

export class OnboardingStatus {
  receivers: boolean;
  budget: boolean;
  topUp: boolean;

  constructor(data?: PoolData) {
    this.receivers = (data && data.receivers.size > 0) || false;
    this.budget =
      (data && data.amountPerBlock.length > 0 && data.amountPerBlock !== "0") ||
      false;
    this.topUp = (data && data.balance > 0) || false;
  }

  isComplete(): boolean {
    return this.receivers && this.budget && this.topUp;
  }
}

/* Receivers */

export type Address = string;

export enum AddressStatus {
  // The address is being added
  Added = "Added",
  // The address is being removed
  Removed = "Removed",
  // The address is present already
  Present = "Present",
}

// Describes a changeset regarding the list of addresses of a pool, i.e.,
// which addresses are being added, removed, or that already exist already.
export type Changeset = Map<string, AddressStatus>;

export function displayAddress(x: Address): string {
  return `${x.slice(0, 8)}...${x.slice(-8)}`;
}