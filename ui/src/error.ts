import * as notification from "./notification";
import * as ipc from "../../native/ipc";

export enum Variant {
  EntityExists = "ENTITY_EXISTS",
  GitError = "GIT_ERROR",
  NotFound = "NOT_FOUND",
}

export interface Error {
  message: string;
  variant: Variant;
}

export const show = (message: string, context: unknown): void => {
  console.error({ message, context });

  notification.error(message, true, "Copy error", () => {
    ipc.copyToClipboard(JSON.stringify({ message, context }, null, 2));
  });
};
