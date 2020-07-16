import * as api from "./api";

/* Offers commodities to query useful information regarding accounts on chain. */

// Get the balance of the account with the given id
const balance = (id: string): Promise<number> =>
  api.get<number>(`accounts/${id}/balance`);

// Check whether an account with the given id exists on chain
const exists = (id: string): Promise<boolean> =>
  api.get<boolean>(`accounts/${id}/exists`);
