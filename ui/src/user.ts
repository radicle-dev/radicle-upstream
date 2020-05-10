import * as api from "./api";
import * as transaction from "./transaction";

// TYPES

// TODO(sos): Avatar should not be optional once users can have avatarUrls
export interface User {
  handle: string;
  maybeEntityId?: string;
}

// EVENTS
interface RegisterInput {
  handle: string;
  maybeEntityId?: string;
}

export const get = (
  handle: string,
): Promise<User | null> => {
  return api.get<User>(`users/${handle}`);
}

export const register = (
  handle: string,
  maybeEntityId?: string,
): Promise<transaction.Transaction> => {
  return api.post<RegisterInput, transaction.Transaction>(`users`, {
    handle, maybeEntityId
  });
}
