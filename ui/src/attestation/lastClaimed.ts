import { writable, Writable } from "svelte/store";

// The last claimed Ethereum address.
// The attestation process has been started, but it may not be completed.
// MUST be lowercase.
export const lastClaimed: Writable<string | undefined> = writable(undefined);
