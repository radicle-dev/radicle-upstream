import { writable } from "svelte/store";

import * as api from "./api";
import * as account from "./account";
import * as currency from "./currency";
import * as transaction from "./transaction";
import * as validation from "./validation";

export const payerStore = writable("");
export const recipientStore = writable("");
export const amountStore = writable("");

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
  fromUser: boolean,
  senderId: string,
  amount: currency.Rad,
  recipient: string,
  transactionFee: number
): Promise<transaction.Transaction> => {
  const endpointBase = fromUser ? "users" : "orgs";
  return api.post<TransferInput, transaction.Transaction>(
    `${endpointBase}/${senderId}/transfer`,
    {
      amount: currency.radToMicroRad(amount),
      recipient,
      transactionFee,
    }
  );
};

export const amountConstraints = {
  presence: {
    message: "The amount is required",
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

const validateRecipientExistence = (accountId: string): Promise<boolean> =>
  account.exists(accountId);

export const recipientConstraints = {
  presence: {
    message: "The recipient address is required",
    allowEmpty: false,
  },
};

export const recipientValidationStore = (): validation.ValidationStore => {
  return validation.createValidationStore(recipientConstraints, [
    {
      promise: validateRecipientExistence,
      validationMessage: "Cannot find this address",
    },
  ]);
};
