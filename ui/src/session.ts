// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { retryOnError } from "ui/src/retryOnError";

import * as proxy from "./proxy";
import * as error from "./error";
import type * as identity from "./identity";
import * as remote from "./remote";
import * as svelteStore from "ui/src/svelteStore";

// TYPES
export enum Status {
  NoSession = "NO_SESSION",
  SealedSession = "SEALED_SESSION",
  UnsealedSession = "UNSEALED_SESSION",
}

export type UnsealedSession = { status: Status.UnsealedSession } & SessionData;

export type Session =
  | { status: Status.NoSession }
  | { status: Status.SealedSession }
  | UnsealedSession;

export interface SessionData {
  identity: identity.Identity;
}

// STATE
const sessionStore: remote.Store<Session> = remote.createStore<Session>();
export const session = sessionStore.readable;

sessionStore.subscribe(data => {
  if (data.status === remote.Status.Error) {
    error.setFatal({
      kind: error.FatalErrorKind.Session,
    });
  }
});

// Return the unseleased session if the session is unsealed. Throw an error
// otherwise.
export const unsealed = (): UnsealedSession => {
  const session = sessionStore.unwrap();
  if (session === undefined || session.status !== Status.UnsealedSession) {
    throw new error.Error({
      code: error.Code.UnsealedSessionExpected,
      message: "session is not unsealed",
      details: {
        session,
      },
    });
  } else {
    return session;
  }
};

// Returns when the session becomes unsealed. Throws when fetching the
// session failed.
export function waitUnsealed(): Promise<UnsealedSession> {
  return svelteStore.waitUntil(sessionStore, data => {
    if (
      data.status === remote.Status.Success &&
      data.data.status === Status.UnsealedSession
    ) {
      return data.data;
    } else if (data.status === remote.Status.Error) {
      throw data.error;
    } else {
      return undefined;
    }
  });
}

// Fetches the session from the proxy and updates the session store.
//
// If `waitUnsealed` is `true` we’ll retry if the session is still
// sealed.
const fetchSession = async (waitUnsealed = false): Promise<void> => {
  try {
    const ses = await retryOnError(
      () => proxy.client.sessionGet(),
      (err: unknown) => {
        if (
          waitUnsealed &&
          err instanceof proxy.ResponseError &&
          err.status === 403
        ) {
          // Session is still sealed but we’re waiting for it to become
          // unsealed.
          return true;
        } else if (err instanceof Error && err.message === "Failed to fetch") {
          // Can’t connect to the proxy—it’s still restaring
          return true;
        } else {
          return false;
        }
      },
      100,
      200 // 20 seconds timeout
    );
    sessionStore.success({ status: Status.UnsealedSession, ...ses });
  } catch (err: unknown) {
    if (err instanceof proxy.ResponseError) {
      if (err.status === 404) {
        sessionStore.success({ status: Status.NoSession });
        return;
      } else if (err.status === 403) {
        sessionStore.success({ status: Status.SealedSession });
        return;
      }
    }

    sessionStore.error(
      new error.Error({
        code: error.Code.SessionFetchFailure,
        message: "Failed to load the session",
        source: err,
      })
    );
  }
};

/**
 * Unseal the key store with the given passphrase and reload the
 * session. Returns `false` if the provided passphrase was incorrect.
 */
export const unseal = async (passphrase: string): Promise<boolean> => {
  try {
    await proxy.client.keyStoreUnseal({ passphrase });
  } catch (err: unknown) {
    if (
      err instanceof proxy.ResponseError &&
      err.variant === "INCORRECT_PASSPHRASE"
    ) {
      return false;
    } else {
      throw err;
    }
  }
  sessionStore.loading();
  await fetchSession(true);
  return true;
};

export const createKeystore = (passphrase: string): Promise<void> => {
  return proxy.client.keyStoreCreate({ passphrase });
};

export const fetch = async (): Promise<void> => {
  sessionStore.loading();
  await fetchSession();
};

export const VALID_SEED_MATCH = /^[\w\d]{54}@([\w\d-]+\.)*[\w\d-]+:[\d]{1,5}$/;

export const __test__ = {
  sessionStore,
  VALID_SEED_MATCH,
};
