import { writable } from "svelte/store";

import * as api from "./api";
import * as currency from "./currency";
import * as transaction from "./transaction";
import * as validation from "./validation";
import { Identity } from "./identity";

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

// / Transfer funds from an account with the given `senderId` to a given `recipient`.
export const transfer = (
  identity: Identity,
  senderId: string,
  amount: currency.MicroRad,
  recipient: string,
  transactionFee: number
): Promise<transaction.Transaction> => {
  const fromUser = senderId === identity.metadata.handle;
  const endpointBase = fromUser ? "users" : "orgs";

  return api.post<TransferInput, transaction.Transaction>(
    `${endpointBase}/${senderId}/transfer`,
    {
      amount,
      recipient,
      transactionFee,
    }
  );
};

export const amountConstraints = {
  presence: {
    message: "Transfer amount is required",
    allowEmpty: false,
  },
  numericality: {
    strict: true,
    greaterThan: 0,
  },
};

export const amountValidationStore = (): validation.ValidationStore => {
  return validation.createValidationStore(amountConstraints);
};
