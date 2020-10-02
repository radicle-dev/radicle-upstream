import { writable } from "svelte/store";

export const txStore = writable<Transaction | null>(null);


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
//TODO(nuno): define this interface
interface Pool { };

//TODO(nuno): define constructor that takes the wallet, instantiated by the Wallet screen.
function make(): Pool {
  return {};
}

export interface Transaction {
  context: string;
  from: string;
  to: string;
  onConfirmed: (value: number) => Promise<void>;
}
