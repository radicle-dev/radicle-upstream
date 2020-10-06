import { writable } from "svelte/store";
import poolCompilerOutput from "radicle-contracts/artifacts/Pool.json";
import { Pool as PoolContract } from "radicle-contracts/contract-bindings/web3/Pool";
import * as web3Utils from "web3-utils";

import { Wallet } from "../wallet";
import * as remote from "../remote";

export const txStore = writable<Transaction | null>(null);

export const store = writable<Pool | null>(null);

// TODO(nuno): Delete this
export interface Transaction {
  context: string;
  from: string;
  to: string;
  onConfirmed: (value: number) => Promise<void>;
}

/*
  Funding
    - Needs to know if there's a connected wallet

    - Collect Funds
      - Need `amount` to be collected
      - Button to collect funds (if any)

    - Pool (if there's connected wallet)
      - Takes all pool info: remaining balance, members list, monthly amount

      - Fill up
        - Prompt Amount (Cancel/Confirm)
        - if confirm, Awaiting user action on the wallet app
          -
      - Save
        - Takes monthly amount and member list
*/

// TODO(nuno): define a better promise return value
export interface Pool {
  data: remote.Store<PoolData>;
  save(data: PoolSettings): Promise<void>;
  fillUp(value: number): Promise<void>;
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

      store.success({
        balance: Number(balance),
        monthlyContribution: 10,
        members: ["0x1", "0x2"],
      });
    } catch (error) {
      store.error(error);
    }
  }

  async function save(_data: PoolSettings): Promise<void> {
    // TODO(thomas): implement this for real using the wallet and the Radicle Contracts
  }

  async function fillUp(value: number): Promise<void> {
    await poolContract.methods.topUp().send({
      value,
    });
  }

  return {
    data: store,
    save,
    fillUp,
  };
}
