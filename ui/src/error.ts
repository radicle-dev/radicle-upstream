import * as notification from "./notification";
import * as ipc from "./ipc";
import * as svelteStore from "svelte/store";

export interface Error {
  // A unique code for easy identification of the error.
  code: Code;
  // Message that is displayed to the user if the error is shown.
  message: string;
  // An optional stack trace
  stack?: string;
  // Arbitrary additional information associated with the error
  details?: unknown;
  // The error that caused this error
  source?: Error;
}

export enum Code {
  BackendTerminated = "BackendTerminated",
  CommitFetchFailure = "CommitFetchFailure",
  KeyStoreUnsealFailure = "KeyStoreUnsealFailure",
  LocalStateFetchFailure = "LocalStateFetchFailure",
  ProjectCheckoutFailure = "ProjectCheckoutFailure",
  ProjectCreationFailure = "ProjectCreationFailure",
  ProjectRequestFailure = "ProjectRequestFailure",
  RemoteStoreError = "RemoteStoreError",
  RequestAbortError = "RequestAbortError",
  SessionFetchFailure = "SessionFetchFailure",
  SessionSettingsUpdateFailure = "SessionSettingsUpdateFailure",
  UnhandledError = "UnhandledError",
  UnhandledRejection = "UnhandledRejection",
  UnknownException = "UnknownException",
}

// Turn a Javascript `Error` into our `Error`.
//
// Uses the code `unknown-exception`.
export const fromException = (exception: globalThis.Error): Error => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const details = Object.assign({}, exception) as any;
  details.name = exception.name;

  let code = Code.UnknownException;
  if (details.name === "AbortError") {
    code = Code.RequestAbortError;
  }

  return {
    code: code,
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

// Fatal application errors that trigger a blue screen
export type FatalError =
  | { kind: FatalErrorKind.Session }
  | { kind: FatalErrorKind.ProxyExit; data: ipc.ProxyError };

export enum FatalErrorKind {
  Session = "SESSION",
  ProxyExit = "PROXY_EXIT",
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
  log({
    code: Code.BackendTerminated,
    message: "Proxy process exited unexpectedly",
    details: { proxyError },
  });
  setFatal({ kind: FatalErrorKind.ProxyExit, data: proxyError });
});

// eslint-disable-next-line @typescript-eslint/no-explicit-any
if (!(window as any).Cypress) {
  window.addEventListener("unhandledrejection", ev => {
    const code = Code.UnhandledRejection;
    const message = "An unexpected error occured";
    if (ev.reason instanceof globalThis.Error) {
      show({
        code,
        message,
        source: fromException(ev.reason),
      });
    } else {
      show({
        code,
        message,
        details: ev.reason,
      });
    }
  });

  window.onerror = (
    _event: unknown,
    _source?: string,
    _lineno?: number,
    _colno?: number,
    error?: globalThis.Error
  ): void => {
    if (error) {
      show({
        code: Code.UnhandledError,
        message: "An unexpected error occured",
        source: fromException(error),
      });
    }
  };
}
// Value is `true` if there was a fatal error
export const fatalError: svelteStore.Readable<FatalError | null> = fatalErrorWritable;
