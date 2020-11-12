import * as notification from "./notification";
import * as ipc from "./ipc";
import * as svelteStore from "svelte/store";

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

const fatalErrorWritable: svelteStore.Writable<boolean> = svelteStore.writable(
  false
);

// Notify the app that there was a fatal error and show the blue screen
// of death.
export const setFatal = (): void => {
  fatalErrorWritable.set(true);
};

ipc.listenProxyError(proxyError => {
  console.error("Proxy process exited", { proxyError });
  setFatal();
});

// Value is `true` if there was a fatal error
export const fatalError: svelteStore.Readable<boolean> = fatalErrorWritable;
