import * as api from "./api";
import * as transaction from "./transaction";

// TYPES

export interface User {
  handle: string;
  maybeProjectId?: string;
}

// EVENTS

interface RegisterInput {
  handle: string;
  maybeProjectId?: string;
}

export const get = (
  handle: string,
): Promise<User | null> => {
  return api.get<User>(`users/${handle}`);
}

export const register = (
  handle: string,
  maybeProjectId?: string,
): Promise<transaction.Transaction> => {
  return api.post<RegisterInput, transaction.Transaction>(`users`, {
    handle, maybeProjectId
  });
}
