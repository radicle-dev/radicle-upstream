import * as api from "./api";
import * as currency from "./currency";
import * as transaction from "./transaction";

// TYPES

export interface User {
  handle: string;
  maybeEntityId?: string;
}

// EVENTS
interface RegisterInput {
  handle: string;
  maybeEntityId?: string;
  transactionFee: currency.MicroRad;
}

export const get = (handle: string): Promise<User | null> => {
  return api.get<User>(`users/${handle}`);
};

export const register = (
  handle: string,
  transactionFee: currency.MicroRad,
  maybeEntityId?: string,
): Promise<transaction.Transaction> => {
  return api.post<RegisterInput, transaction.Transaction>(`users`, {
    handle,
    transactionFee,
    maybeEntityId,
  });
};
