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

// Fatal application errors that trigger a blue screen
export type FatalError =
  | { kind: FatalErrorKind.SESSION }
  | { kind: FatalErrorKind.PROXY_EXIT; data: ipc.ProxyError };

export enum FatalErrorKind {
  SESSION = "SESSION",
  PROXY_EXIT = "PROXY_EXIT",
}

const fatalErrorWritable: svelteStore.Writable<FatalError | null> = svelteStore.writable(
  null
);

// Notify the app that there was a fatal error and show the blue screen
// of death.
export const setFatal = (fatalError: FatalError): void => {
  fatalErrorWritable.set(fatalError);
};

ipc.listenProxyError(proxyError => {
  console.error("Proxy process exited", { proxyError });
  setFatal({ kind: FatalErrorKind.PROXY_EXIT, data: proxyError });
});

// Value is `true` if there was a fatal error
export const fatalError: svelteStore.Readable<FatalError | null> = fatalErrorWritable;
