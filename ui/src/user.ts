import * as api from "./api";

// TYPES

export interface User {
  handle: string;
  maybeEntityId?: string;
}

export const get = (handle: string): Promise<User | null> => {
  return api.get<User>(`users/${handle}`);
};
