import { writable } from "svelte/store";

export const payerStore = writable(String);
export const recipientStore = writable(String);
export const amountStore = writable(String);

export enum TransferState {
  Preparation,
  Confirmation,
}
