import { writable } from "svelte/store";
import poolCompilerOutput from "radicle-contracts/artifacts/Pool.json";
import { Pool as PoolContract } from "radicle-contracts/contract-bindings/web3/Pool";
import * as web3Utils from "web3-utils";

import { Wallet } from "../wallet";
import * as remote from "../remote";

export const store = writable<Pool | null>(null);

export interface Pool {
  data: remote.Store<PoolData>;
  save(data: PoolSettings): Promise<void>;
  topUp(value: number): Promise<void>;
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

// TODO(nuno|thomas): Better define this once we get to use the underlying functions.
export function make(wallet: Wallet): Pool {
  const store = remote.createStore<PoolData>();
  // TODO(nuno|thomas): actually load the pool data from the pool contract.

  const poolContract = (new wallet.web3.eth.Contract(
    (poolCompilerOutput.abi as unknown) as web3Utils.AbiItem[],
    "0x0e22b57c7e69d1b62c9e4c88bb63b0357a905d1e"
  ) as unknown) as PoolContract;

  loadPoolData();

  async function loadPoolData() {
    try {
      const balance = await poolContract.methods.withdrawable().call();
      const collectableFunds = await poolContract.methods.collectable().call();

      store.success({
        // Handle potential overflow using BN.js
        balance: Number(balance),
        monthlyContribution: 10,
        members: ["0x1", "0x2"],
        // Handle potential overflow using BN.js
        collectableFunds: Number(collectableFunds),
      });
    } catch (error) {
      store.error(error);
    }
  }

  async function save(_data: PoolSettings): Promise<void> {
    throw new Error("not implemented");
  }

  async function topUp(value: number): Promise<void> {
    await poolContract.methods.topUp().send({
      value,
    });
  }

  async function collect(): Promise<void> {
    throw new Error("not implemented");
  }

  return {
    data: store,
    save,
    topUp,
    collect,
  };
}
