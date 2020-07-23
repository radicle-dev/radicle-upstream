import { writable } from "svelte/store";

export const fromStore = writable(String);
export const toStore = writable(String);
export const amountStore = writable(String);
