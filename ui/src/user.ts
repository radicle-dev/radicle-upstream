import * as api from "./api";
import * as transaction from "./transaction";

// TYPES

export interface User {
  handle: string;
  maybeId?: string;
}

// EVENTS

interface RegisterInput {
  handle: string;
  maybeId?: string;
}

export const get = (
  handle: string,
): Promise<User | null> => {
  return api.get<User>(`users/${handle}`);
}

export const register = (
  handle: string,
  maybeId?: string,
): Promise<transaction.Transaction> => {
  return api.post<RegisterInput, transaction.Transaction>(`users`, {
    handle, maybeId
  });
}
