import { writable } from "svelte/store";

import { Wallet } from "../wallet";
import * as remote from "../remote";

export const txStore = writable<Transaction | null>(null);

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
  store.success({
    balance: 10,
    monthlyContribution: 10,
    members: ["0x1", "0x2"],
  });

  async function save(data: PoolSettings): Promise<void> {
    // TODO(thomas): implement this for real using the wallet and the Radicle Contracts
    return wallet.testSign(JSON.stringify(data));
  }

  async function fillUp(value: number): Promise<void> {
    // TODO(thomas): implement this for real using the wallet and the Radicle Contracts
    return wallet.testTransfer(value);
  }

  return {
    data: store,
    save,
    fillUp,
  };
}
