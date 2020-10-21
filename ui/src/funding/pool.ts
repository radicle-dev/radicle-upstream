import * as svelteStore from "svelte/store";
import { PoolFactory } from "radicle-contracts/build/contract-bindings/ethers/PoolFactory";
import { Pool as PoolContract } from "radicle-contracts/contract-bindings/ethers/Pool";

import * as transaction from "../transaction";
import * as validation from "../validation";

import { Wallet, Account, Status } from "../wallet";
import * as remote from "../remote";
import { BigNumberish } from "ethers";

export const store = svelteStore.writable<Pool | null>(null);

export interface Pool {
  data: remote.Store<PoolData>;

  // Get the account that owns this pool. Should be the connected wallet account.
  getAccount: () => Account | undefined;

  // Update the contribution amount per block. Returns once the
  // transaction has been included in the chain.
  updateAmountPerBlock(amountPerBlock: string): Promise<void>;

  // Update the list of receiver addresses. Returns once the
  // transaction has been included in the chain.
  updateReceiverAddresses(data: PoolData, addresses: string[]): Promise<void>;

  // Adds funds to the pool. Returns once the transaction has been
  // included in the chain.
  topUp(value: number): Promise<void>;
  // Collect funds the user has received up to now from givers and
  // transfer them to the users account.
  collect(): Promise<void>;
}

// The pool settings the user can change and save.
export interface PoolSettings {
  // The total amount to be disbursed to all receivers with each block.
  amountPerBlock: string;
  // The list of addresses that receive funds from the pool.
  receiverAddresses: string[];
}

// All the data representing a pool.
export interface PoolData {
  // The remaining balance of this pool.
  balance: number;
  // The total amount to be disbursed to all receivers with each block.
  amountPerBlock: string;
  // The list of addresses that receive funds from the pool.
  receiverAddresses: string[];
  // Funds that the user can collect from their givers.
  collectableFunds: number;
}

export function make(wallet: Wallet): Pool {
  const data = remote.createStore<PoolData>();
  const address = "0x0e22b57c7e69d1b62c9e4c88bb63b0357a905d1e";
  const poolContract: PoolContract = PoolFactory.connect(
    address,
    wallet.signer
  );

  loadPoolData();

  async function loadPoolData() {
    try {
      const balance = await poolContract.withdrawable();
      const collectableFunds = await poolContract.collectable();
      const amountPerBlock = await poolContract.getAmountPerBlock();
      const receivers = await poolContract.getAllReceivers();
      const receiverAddresses = receivers.map(r => r.receiver);

      data.success({
        // Handle potential overflow using BN.js
        balance: Number(balance),
        amountPerBlock: amountPerBlock.toString(),
        receiverAddresses,
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

  async function updateReceiverAddresses(
    data: PoolData,
    addresses: string[]
  ): Promise<void> {
    // TODO(nuno): Read instance `data` instead of receiving as an argument.
    const newAddresses = addresses.filter(
      x => !data.receiverAddresses.includes(x)
    );
    const txs = newAddresses.map(address =>
      poolContract.setReceiver(address, 1).then(tx => {
        transaction.add(transaction.beneficiaries(tx));
        tx.wait();
      })
    );

    // TODO check transaction status
    await Promise.all(txs).finally(loadPoolData);
  }

  async function topUp(value: number): Promise<void> {
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
    updateAmountPerBlock,
    updateReceiverAddresses,
    topUp,
    collect,
  };
}

/**
 * Stores
 */
export const membersStore = svelteStore.writable("");
export const amountStore = svelteStore.writable("");

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

  // The contraints for a valid amount input.
  amount: {
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

export const amountValidationStore = (
  balance: BigNumberish
): validation.ValidationStore => {
  return validation.createValidationStore(contraints.amount, [
    {
      promise: amount => Promise.resolve(balance >= amount),
      validationMessage: "Insufficient balance",
    },
  ]);
};
