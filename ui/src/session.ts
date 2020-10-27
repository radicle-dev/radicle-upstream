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
export interface Authenticated {
  identity: identity.Identity;
  settings: Settings;
}

export interface Unauthenticated {
  settings: Settings;
}

export type Session = Authenticated | Unauthenticated;

// STATE
const sessionStore = remote.createStore<Session | null>();
export const session = sessionStore.readable;

export const settings: Readable<Settings> = derived(sessionStore, sess => {
  if (
    sess.status === remote.Status.Success &&
    sess.data &&
    (<Session>sess.data).settings
  ) {
    return (<Session>sess.data).settings;
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

const handleSessionError = async (error: Error): Promise<Session | null> => {
  if (error instanceof api.ResponseError) {
    if (error.response.status === 404) {
      return null;
    } else if (error.response.status === 403) {
      return {} as Session;
    } else {
      throw error;
    }
  } else {
    throw error;
  }
};

const fetchSession = (): Promise<void> =>
  api
    .get<Session>(`session`)
    .catch(error => handleSessionError(error))
    .then(sessionStore.success)
    .catch(sessionStore.error);

export const unseal = (passphrase: string): Promise<void> => {
  sessionStore.loading();
  return api
    .set<unknown>(`session/unseal`, { passphrase })
    .catch((err: error.Error) => {
      if (error instanceof api.ResponseError && error.response.status === 403) {
        sessionStore.success({} as Session);
      } else {
        notification.error(`Could not unlock the session: ${err.message}`);
      }
    })
    .then(fetchSession);
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
