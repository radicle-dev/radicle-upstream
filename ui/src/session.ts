import { Readable, derived, get } from "svelte/store";

import * as api from "./api";
import * as error from "./error";
import * as event from "./event";
import * as identity from "./identity";
import * as notification from "./notification";
import * as remote from "./remote";
import { Appearance, CoCo, Settings, defaultSetttings } from "./settings";

import { createValidationStore, ValidationStatus } from "./validation";

// TYPES
export enum Status {
  NoSession = "No_SESSION",
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
  Clear = "CLEAR",
  ClearCache = "CLEAR_CACHE",
  Fetch = "FETCH",
  UpdateSettings = "UPDATE_SETTINGS",
}

interface Clear extends event.Event<Kind> {
  kind: Kind.Clear;
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

type Msg = Clear | ClearCache | Fetch | UpdateSettings;

const fetchSessionRetry = async () => {
  return api
    .withRetry(() => api.get<SessionData>(`session`), 200)
    .then(ses =>
      sessionStore.success({ status: Status.UnsealedSession, ...ses })
    )
    .catch(async error => {
      if (error instanceof api.ResponseError) {
        if (error.response.status === 404) {
          sessionStore.success({ status: Status.NoSession });
        } else if (error.response.status === 403) {
          sessionStore.success({ status: Status.SealedSession });
        } else {
          throw error;
        }
      } else {
        throw error;
      }
    });
};

const fetchSession = (): Promise<void> => {
  return fetchSessionRetry().catch(sessionStore.error);
};

export const unseal = async (passphrase: string): Promise<void> => {
  sessionStore.loading();
  try {
    await api.set<unknown>(`keystore/unseal`, { passphrase });
  } catch (error) {
    sessionStore.success({ status: Status.SealedSession });
    notification.error(`Could not unlock the session: ${error.message}`);
    return;
  }
  notification.info("Unsealing the session...");
  await fetchSession();
};

export const createKeystore = (): Promise<null> => {
  return api.set<unknown>(`keystore`, {});
};

const updateSettings = (settings: Settings): Promise<void> =>
  api
    .set<Settings>(`session/settings`, settings)
    .then(fetchSession)
    .catch((err: error.Error) => notification.error(err.message));

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.Clear:
      api
        .del(`session`)
        .then(fetchSession)
        .catch(reason => {
          console.error("DEL session failed: ", reason);
        });

      break;

    case Kind.Fetch:
      sessionStore.loading();
      fetchSession().catch(reason => {
        console.error("fetchSession() failed: ", reason);
      });

      break;

    case Kind.UpdateSettings:
      updateSettings(msg.settings).catch(reason => {
        console.error("updateSettings() failed: ", reason);
      });

      break;
  }
};

export const clear = event.create<Kind, Msg>(Kind.Clear, update);
export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);

export const updateAppearance = (appearance: Appearance): void =>
  event.create<Kind, Msg>(
    Kind.UpdateSettings,
    update
  )({
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
    settings: { ...get(settings), appearance },
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

const VALID_SEED_MATCH = /^[\w\d]{54}@[\w\d.]*:[\d]{1,5}$/;

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
