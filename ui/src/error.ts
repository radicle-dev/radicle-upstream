import * as notification from "./notification";
import * as ipc from "./ipc";
import * as svelteStore from "svelte/store";

export interface Error {
  // A code in kebab-case for easy identification of the error.
  //
  // This should not include interpolated data.
  code: string;
  // Message that is displayed to the user if the error is shown.
  message: string;
  // An optional stack trace
  stack?: string;
  // Arbitrary additional information associated with the error
  details?: unknown;
  // The error that caused this error
  source?: Error;
}

// Turn a Javascript `Error` into our `Error`.
//
// Uses the code `unknown-exception`.
export const fromException = (exception: globalThis.Error): Error => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const details = Object.assign({}, exception) as any;
  details.name = exception.name;
  return {
    code: "unknown-exception",
    message: exception.message,
    stack: exception.stack,
    details,
  };
};

// Log an error to the console.
export const log = (error: Error): void => {
  const { message, ...rest } = error;
  console.error(message, rest);
};

// Show an error notification and log the error to the console
export const show = (error: Error): void => {
  log(error);

  notification.error(error.message, true, "Copy error", () => {
    ipc.copyToClipboard(JSON.stringify(error, null, 2));
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
  log({
    code: "unexpected-proxy-exit",
    message: "Proxy process exicted unexpectedly",
    details: { proxyError },
  });
  setFatal();
});

// Value is `true` if there was a fatal error
export const fatalError: svelteStore.Readable<boolean> = fatalErrorWritable;
