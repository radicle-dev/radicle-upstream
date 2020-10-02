import { writable } from "svelte/store";

import { Wallet } from "../wallet";

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
  save(data: PoolData): Promise<Result>;
  fillUp(value: number): Promise<Result>;
}

export interface PoolData {
  // The amount to be disbursed monthly.
  monthlyContribution: number;
  // The list of eth addresses across whom the
  // `monthlyContribution` is evenly spread.
  members: string[];
}

enum Error {
  UserRejected,
  OtherError,
}

// TODO(nuno|thomas): Better define this once we get to use the underlying functions.
export type Result =
  | { type: "Error"; error: Error }
  | { type: "Ok"; value: any };

function ok(value: any): Result {
  return { type: "Ok", value };
}

function err(error: Error): Result {
  return { type: "Error", error };
}

export function make(wallet: Wallet): Pool {
  async function save(data: PoolData): Promise<Result> {
    // TODO(thomas): implement this for real using the wallet and the Radicle Contracts
    try {
      const response = await wallet.testSign(JSON.stringify(data));
      return ok(response);
    } catch (e) {
      return err(e);
    }
  }

  async function fillUp(value: number): Promise<Result> {
    // TODO(thomas): implement this for real using the wallet and the Radicle Contracts
    try {
      const v = await wallet.testTransfer(10);
      return ok(v);
    } catch (e) {
      return err(e);
    }
  }

  return {
    save,
    fillUp,
  };
}
