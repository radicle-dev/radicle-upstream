import { writable } from "svelte/store";

export const txStore = writable<Transaction | null>(null);

export interface Transaction {
  context: string;
  from: string;
  to: string;
  onConfirmed: (value: number) => Promise<void>;
}
