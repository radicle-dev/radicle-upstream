import { writable } from "svelte/store";

import * as api from "./api";
import * as currency from "./currency";
import * as transaction from "./transaction";

export const payerStore = writable(String);
export const recipientStore = writable(String);
export const amountStore = writable(String);

export enum TransferState {
  Preparation,
  Confirmation,
  Sent,
}

interface TransferInput {
  amount: currency.MicroRad;
  recipient: string;
  transactionFee: number;
}

// / Transfer funds from a user account to a given recipient
export const transfer = (
  handle: string,
  amount: currency.MicroRad,
  recipient: string,
  transactionFee: number
): Promise<transaction.Transaction> => {
  return api.post<TransferInput, transaction.Transaction>(
    `users/${handle}/transfer`,
    {
      amount,
      recipient,
      transactionFee,
    }
  );
};
