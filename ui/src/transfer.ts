import { writable } from "svelte/store";

import * as api from "./api";
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

// TODO(nuno): delete or repurpose to use wallet
const validateSufficientBalance = (fee: number, payerAccountId: string) => (
  amount: string
): Promise<boolean> => {
  return Promise.resolve(true);
};

// TODO(nuno): delete or repurpose to use wallet
export const amountValidationStore = (
  payer: string,
  amount: number
): validation.ValidationStore => {
  if (payer && payer.length > 0) {
    return validation.createValidationStore(amountConstraints, [
      {
        promise: validateSufficientBalance(amount, payer),
        validationMessage:
          "You don't have enough funds in this wallet for this transfer",
      },
    ]);
  } else {
    return validation.createValidationStore(amountConstraints);
  }
};

export const recipientConstraints = {
  presence: {
    message: "The recipient address is required",
    allowEmpty: false,
  },
};

const validateRecipientIsNotSender = (senderAccountId?: string) => (
  recipientAccountId: string
): Promise<boolean> => Promise.resolve(senderAccountId !== recipientAccountId);

export const recipientValidationStore = (
  senderAccountId?: string
): validation.ValidationStore => {
  return validation.createValidationStore(recipientConstraints, [
    {
      promise: validateRecipientIsNotSender(senderAccountId),
      validationMessage: "Cannot make a transfer to the same account",
    },
  ]);
};
