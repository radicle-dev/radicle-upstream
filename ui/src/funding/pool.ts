import { writable } from "svelte/store";
import poolCompilerOutput from "radicle-contracts/artifacts/Pool.json";
import { Pool as PoolContract } from "radicle-contracts/contract-bindings/web3/Pool";
import * as web3Utils from "web3-utils";
import * as svelteStore from "svelte/store";

import { Wallet, Status as WalletStatus } from "../wallet";
import * as remote from "../remote";

export const store = writable<Pool | null>(null);

export interface Pool {
  data: remote.Store<PoolData>;
  // Save the pool settings. Returns once the transaction has been
  // included in the chain.
  save(data: PoolSettings): Promise<void>;
  // Adds funds to the pool. Returns once the transaction has been
  // included in the chain.
  topUp(value: number): Promise<void>;
  // Collect funds the user has received up to now from givers and
  // transfer them to the users account.
  collect(): Promise<void>;
}

// The pool settings the user can change and save.
export interface PoolSettings {
  // The amount to be disbursed monthly.
  monthlyContribution: number;
  // The list of eth addresses across whom the
  // `monthlyContribution` is evenly spread.
  members: string[];
}

// All the data representing a pool.
export interface PoolData {
  // The remaining balance of this pool.
  balance: number;
  // The amount to be disbursed monthly.
  monthlyContribution: number;
  // The list of eth addresses across whom the
  // `monthlyContribution` is evenly spread.
  members: string[];
  // Funds that the user can collect from their givers.
  collectableFunds: number;
}

export function make(wallet: Wallet): Pool {
  const data = remote.createStore<PoolData>();

  const poolContract = (new wallet.web3.eth.Contract(
    (poolCompilerOutput.abi as unknown) as web3Utils.AbiItem[],
    "0x0e22b57c7e69d1b62c9e4c88bb63b0357a905d1e"
  ) as unknown) as PoolContract;

  loadPoolData();

  async function loadPoolData() {
    try {
      const balance = await poolContract.methods.withdrawable().call();
      const collectableFunds = await poolContract.methods.collectable().call();
      console.log(collectableFunds);

      data.success({
        // Handle potential overflow using BN.js
        balance: Number(balance),
        monthlyContribution: 10,
        members: ["0x1", "0x2"],
        // Handle potential overflow using BN.js
        collectableFunds: Number(collectableFunds),
      });
    } catch (error) {
      data.error(error);
    }
  }

  async function save(_data: PoolSettings): Promise<void> {
    throw new Error("not implemented");
  }

  async function topUp(value: number): Promise<void> {
    await poolContract.methods.topUp().send({
      from: getAccountAddress(),
      value,
    });
    loadPoolData();
  }

  async function collect(): Promise<void> {
    await poolContract.methods.collect().send({
      from: getAccountAddress(),
    });
    loadPoolData();
  }

  // Get the account address from the wallet. Throws an error if we’re
  // not connected.
  function getAccountAddress() {
    const state = svelteStore.get(wallet);
    if (state.status === WalletStatus.Connected) {
      return state.connected.account.address;
    } else {
      throw new Error("Wallet is not connected");
    }
  }

  return {
    data,
    save,
    topUp,
    collect,
  };
}
