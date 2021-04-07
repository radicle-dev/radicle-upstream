import { Readable, derived, get } from "svelte/store";

import * as proxy from "./proxy";
import * as error from "./error";
import * as event from "./event";
import type * as identity from "./identity";
import * as remote from "./remote";
import {
  Appearance,
  CoCo,
  Settings,
  defaultSetttings,
  FeatureFlags,
} from "./settings";

import { createValidationStore, ValidationStatus } from "./validation";

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
  settings: Settings;
}

// STATE
const sessionStore = remote.createStore<Session>();
export const session = sessionStore.readable;

sessionStore.subscribe(data => {
  if (data.status === remote.Status.Error) {
    error.setFatal({
      kind: error.FatalErrorKind.Session,
    });
  }
});

// Returns when the session becomes unsealed. Throws when fetching the
// session failed.
export const waitUnsealed = (): Promise<void> => {
  return new Promise((resolve, reject) => {
    let resolvedNow = false;
    let unsubscribe: () => void | undefined;
    // We’re using `let` so that we can access `unsubscribe` if the
    // `susbscribe` callback is called synchronously.
    // eslint-disable-next-line prefer-const
    unsubscribe = sessionStore.subscribe(data => {
      if (
        data.status === remote.Status.Success &&
        data.data.status === Status.UnsealedSession
      ) {
        if (unsubscribe) {
          unsubscribe();
        } else {
          resolvedNow = true;
        }
        resolve();
      } else if (data.status === remote.Status.Error) {
        reject(data.error);
      }
    });
    if (resolvedNow) {
      unsubscribe();
    }
  });
};

export const settings: Readable<Settings> = derived(sessionStore, sess => {
  if (
    sess.status === remote.Status.Success &&
    sess.data.status === Status.UnsealedSession
  ) {
    return sess.data.settings;
  } else {
    return defaultSetttings();
  }
});

// EVENTS
enum Kind {
  ClearCache = "CLEAR_CACHE",
  Fetch = "FETCH",
  UpdateSettings = "UPDATE_SETTINGS",
}

interface ClearCache extends event.Event<Kind> {
  kind: Kind.ClearCache;
}

interface Fetch extends event.Event<Kind> {
  kind: Kind.Fetch;
}

interface UpdateSettings extends event.Event<Kind> {
  kind: Kind.UpdateSettings;
  settings: Settings;
}

type Msg = ClearCache | Fetch | UpdateSettings;

const fetchSession = async (): Promise<void> => {
  try {
    const ses = await proxy.withRetry(() => proxy.client.sessionGet(), 100, 50);
    sessionStore.success({ status: Status.UnsealedSession, ...ses });
  } catch (err) {
    if (err instanceof proxy.ResponseError) {
      if (err.response.status === 404) {
        sessionStore.success({ status: Status.NoSession });
        return;
      } else if (err.response.status === 403) {
        sessionStore.success({ status: Status.SealedSession });
        return;
      }
    }

    sessionStore.error({
      code: error.Code.SessionFetchFailure,
      message: "Failed to load the session",
      source: error.fromException(err),
    });
  }
};

/**
 * Unseal the key store with the given passphrase and reload the
 * session. Returns `true` if unsealing was successful and `false`
 * otherwise.
 */
export const unseal = async (passphrase: string): Promise<boolean> => {
  try {
    await proxy.client.keyStoreUnseal({ passphrase });
  } catch (err) {
    error.show({
      code: error.Code.KeyStoreUnsealFailure,
      message: `Could not unlock the session: ${err.message}`,
      source: err,
    });

    return false;
  }
  sessionStore.loading();
  await fetchSession();
  return true;
};

export const createKeystore = (passphrase: string): Promise<void> => {
  return proxy.client.keyStoreCreate({ passphrase });
};

const updateSettings = async (settings: Settings): Promise<void> => {
  try {
    await proxy.client.sessionSettingsSet(settings);
  } catch (err) {
    error.show({
      code: error.Code.SessionSettingsUpdateFailure,
      message: `Failed to update settings: ${err.message}`,
      source: error.fromException(err),
    });
    return;
  }

  await fetchSession();
};

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.Fetch:
      sessionStore.loading();
      fetchSession();
      break;

    case Kind.UpdateSettings:
      updateSettings(msg.settings);
      break;
  }
};

export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);

export const updateAppearance = (appearance: Appearance): void =>
  event.create<Kind, Msg>(
    Kind.UpdateSettings,
    update
  )({
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
    settings: { ...get(settings), appearance },
  });

export const updateFeatureFlags = (featureFlags: FeatureFlags): void =>
  event.create<Kind, Msg>(
    Kind.UpdateSettings,
    update
  )({
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
    settings: { ...get(settings), featureFlags },
  });

export const dismissRemoteHelperHint = (): void => {
  updateAppearance({
    // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
    ...get(settings).appearance,
    hints: { showRemoteHelper: false },
  });
};

export const updateCoCo = (coco: CoCo): void =>
  event.create<Kind, Msg>(
    Kind.UpdateSettings,
    update
  )({
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
    settings: { ...get(settings), coco },
  });

const VALID_SEED_MATCH = /^[\w\d]{54}@([\w\d-]+\.)*[\w\d-]+:[\d]{1,5}$/;

const checkSeedUniqueness = (seed: string): Promise<boolean> => {
  return Promise.resolve(!get(settings).coco.seeds.includes(seed));
};

export const seedValidation = createValidationStore(
  {
    format: {
      pattern: VALID_SEED_MATCH,
      message: "This is not a valid seed address",
    },
  },
  [
    {
      promise: checkSeedUniqueness,
      validationMessage: "This seed already exists",
    },
  ]
);

export const addSeed = async (seed: string): Promise<boolean> => {
  // This has to be awaited contrary to what tslint suggests, because we're
  // running async remote validations in in the background. If we remove the
  // async then the seed input form will have to be submitted twice to take any
  // effect.
  await seedValidation.validate(seed);
  if (get(seedValidation).status !== ValidationStatus.Success) return false;

  updateCoCo({ seeds: [...get(settings).coco.seeds, seed] });
  return true;
};

export const removeSeed = (seed: string): void => {
  updateCoCo({
    seeds: get(settings).coco.seeds.filter((x: string) => x !== seed),
  });
  seedValidation.reset();
};

export const __test__ = {
  sessionStore,
  VALID_SEED_MATCH,
};
