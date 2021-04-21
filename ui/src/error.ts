import * as svelteStore from "svelte/store";
import * as lodash from "lodash";

import * as notification from "./notification";
import * as ipc from "./ipc";

interface ErrorParams {
  // A unique code for easy identification of the error.
  code: Code;
  // Message that is displayed to the user if the error is shown.
  message: string;
  // Arbitrary additional information associated with the error
  details?: unknown;
  // The underlying source of this error. This is usually another error
  // that was thrown and is wrapped.
  //
  // The constructor calls `fromUnknown` on this value to set
  // `Error.source`.
  source?: unknown;
}

// An extension of the built-in JavaScript `Error` that includes more
// contextual information.
export class Error extends globalThis.Error {
  // A unique code for easy identification of the error.
  readonly code: Code;
  // Message that is displayed to the user if the error is shown.
  readonly message: string;
  // An optional stack trace
  stack?: string;
  // Arbitrary additional information associated with the error
  readonly details?: unknown;
  // The error that caused this error
  readonly source?: Error;

  constructor(params: ErrorParams) {
    super(params.message);
    this.message = params.message;
    this.code = params.code;
    this.details = params.details;
    if (params.source) {
      this.source = fromUnknown(params.source);
    }
  }
}

export enum Code {
  BackendTerminated = "BackendTerminated",
  CommitFetchFailure = "CommitFetchFailure",
  IdentityCreationFailure = "IdentityCreationFailure",
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
  UnsealedSessionExpected = "UnsealedSessionExpected",

  // Funding related error codes
  WalletConnectionFailure = "WalletConnectionFailure",
  FailedOrRejectedTransaction = "FailedOrRejectedTransaction",
  UnkownTransactionFailure = "UnkownTransactionFailure",
  InsufficientGas = "InsufficientGas",

  // Custom protocol error codes
  CustomProtocolUnsupportedVersion = "CustomProtocolUnsupportedVersion",
  CustomProtocolUnsupportedNamespace = "CustomProtocolUnsupportedNamespace",
  CustomProtocolParseError = "CustomProtocolParseError",
}

// Turn a built-in Javascript `Error` into our `Error`.
//
// Uses the code `UnknownException`, assigns all properties of
// `originalError` to `Error.details` and preserves the stacktrace.
export const fromJsError = (
  originalError: globalThis.Error,
  code: Code = Code.UnknownException
): Error => {
  const details = Object.assign(
    {
      // `name` might only be defined on the prototype of
      // `originalError` so we need to add it explicitly
      name: originalError.name,
    },
    lodash.omit(originalError, ["message", "stack"])
  );

  if (details.name === "AbortError") {
    code = Code.RequestAbortError;
  }

  const error = new Error({
    code: code,
    message: originalError.message,
    details,
  });

  error.stack = originalError.stack;

  return error;
};

// Creates an error from an unknown value. This is useful in
// `try/catch` statements or `Promise.catch()` where nothing is known
// about the error value.
//
// If `value` is already an instance of of `Error`, it is returned.
//
// If `value` is an instance of the built-in JavaScript `Error`, it is
// converted with `fromJsError` using `fallbackCode` as `Error.code`.
//
// Otherwise, an error with the given fallback code and message is
// created and `value` is assigned to `Error.details`.
export function fromUnknown(
  value: unknown,
  fallbackCode: Code = Code.UnknownException,
  fallbackMessage: string = "An unexpected error occured"
): Error {
  if (value instanceof Error) {
    return value;
  } else if (value instanceof globalThis.Error) {
    return fromJsError(value, fallbackCode);
  } else {
    return new Error({
      code: fallbackCode,
      message: fallbackMessage,
      details: value,
    });
  }
}

// Log an error to the console for inspection by developers.
export const log = (error: Error): void => {
  const info: { details?: unknown; source?: Error } = {};
  if (error.details) {
    info.details = error.details;
  }
  if (error.source) {
    info.source = error.source;
  }
  console.error(error, info);
};

// Show an error notification and log the error to the console
export const show = (error: Error): void => {
  log(error);

  notification.error({
    message: error.message,
    showIcon: true,
    actions: [
      {
        label: "Copy error",
        handler: () => {
          ipc.copyToClipboard(JSON.stringify(error, null, 2));
        },
      },
    ],
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
  log(
    new Error({
      code: Code.BackendTerminated,
      message: "Proxy process exited unexpectedly",
      details: { proxyError },
    })
  );
  setFatal({ kind: FatalErrorKind.ProxyExit, data: proxyError });
});

// eslint-disable-next-line @typescript-eslint/no-explicit-any
if (!(window as any).Cypress) {
  window.addEventListener("unhandledrejection", ev => {
    ev.preventDefault();
    show(fromUnknown(ev.reason, Code.UnhandledRejection));
  });

  window.addEventListener("error", ev => {
    ev.preventDefault();
    show(fromUnknown(ev.error, Code.UnhandledError));
  });
}

// Value is `true` if there was a fatal error
export const fatalError: svelteStore.Readable<FatalError | null> = fatalErrorWritable;
