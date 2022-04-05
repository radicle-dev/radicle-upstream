// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as svelteStore from "svelte/store";
import lodash from "lodash";

import * as bacon from "./bacon";
import * as ipc from "./ipc";

const notificationBus = new bacon.Bus<Error>();
export const notifications: bacon.EventStream<Error> =
  notificationBus.toEventStream();
export function showNotification(error: Error): void {
  notificationBus.push(error);
}

interface ErrorParams {
  // A unique code for easy identification of the error.
  code?: Code;
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
  public readonly code?: Code;
  // Message that is displayed to the user if the error is shown.
  public readonly message: string;
  // An optional stack trace
  public stack?: string;
  // Arbitrary additional information associated with the error
  public readonly details?: unknown;
  // The error that caused this error
  public readonly source?: Error;

  public constructor(params: ErrorParams) {
    super(params.message);
    Object.defineProperty(this, "stack", {
      enumerable: true,
    });
    this.code = params.code;
    this.message = params.message;
    Object.defineProperty(this, "message", {
      enumerable: true,
    });
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
  ProxyEventParseFailure = "ProxyEventParseFailure",
  KeyStoreUnsealFailure = "KeyStoreUnsealFailure",
  LocalStateFetchFailure = "LocalStateFetchFailure",
  ProjectRequestFailure = "ProjectRequestFailure",
  RemoteStoreError = "RemoteStoreError",
  RequestAbortError = "RequestAbortError",
  SessionFetchFailure = "SessionFetchFailure",
  UpdateEthereumClaimFailure = "UpdateEthereumClaimFailure",
  SessionSettingsUpdateFailure = "SessionSettingsUpdateFailure",
  UnhandledError = "UnhandledError",
  UnhandledRejection = "UnhandledRejection",
  UnknownException = "UnknownException",
  Unreachable = "Unreachable",
  UnsealedSessionExpected = "UnsealedSessionExpected",
  EmptyHistory = "EmptyHistory",

  // Ethereum related error codes
  WalletConnectionFailure = "WalletConnectionFailure",
  FailedOrRejectedTransaction = "FailedOrRejectedTransaction",
  UnkownTransactionFailure = "UnkownTransactionFailure",
  InsufficientGas = "InsufficientGas",

  // Org related error codes
  OrgCreateCouldNotGenerateTx = "OrgCreateCouldNotGenerateTx",
  OrgCreateNotFoundInInterfaceLogs = "OrgCreateNotFoundInInterfaceLogs",
  OrgFetchOrgsCalledWithNoWallet = "OrgFetchOrgsCalledWithNoWallet",
  OrgIdentitySha1UrnError = "OrgIdentitySha1UrnError",
  OrgCreatationFailed = "OrgCreatationFailed",
  OrgAnchorCreatationFailed = "OrgAnchorCreatationFailed",
  OrgFetchFailed = "OrgFetchFailed",

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

// Fatal application errors that trigger a blue screen
export type FatalError =
  | { kind: FatalErrorKind.Session }
  | { kind: FatalErrorKind.ProxyExit; data: ipc.ProxyError };

export enum FatalErrorKind {
  Session = "SESSION",
  ProxyExit = "PROXY_EXIT",
}

const fatalErrorWritable: svelteStore.Writable<FatalError | null> =
  svelteStore.writable(null);

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

// Value is `true` if there was a fatal error
export const fatalError: svelteStore.Readable<FatalError | null> =
  fatalErrorWritable;
