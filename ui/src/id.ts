import * as org from "./org";
import * as user from "./user";

// Check if the given id is already taken
export const isTaken = (id: string): Promise<boolean> =>
  org.getOrg(id).then(org => !!org || user.get(id).then(user => !!user));

// Check if the given id is available
export const isAvailable = (id: string): Promise<boolean> =>
  isTaken(id).then(taken => !taken);
