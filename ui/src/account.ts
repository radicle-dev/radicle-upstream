import * as api from "./api";
import * as remote from "./remote";

/* Offers commodities to query useful information regarding accounts on chain. */

// Get the balance of the account with the given id
export const getBalance = (id: string): Promise<number> =>
  api.get<number>(`accounts/${id}/balance`);

// Check whether an account with the given id exists on chain
export const exists = (id: string): Promise<boolean> =>
  api.get<boolean>(`accounts/${id}/exists`);

// State
const balanceStore = remote.createStore<number>();
export const balance = balanceStore.readable;

export const updateBalance = (id: string) => {
  balanceStore.loading();
  getBalance(id).then(balanceStore.success).catch(balanceStore.error);
};
