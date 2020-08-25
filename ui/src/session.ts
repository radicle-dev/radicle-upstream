import { Readable, derived, get } from "svelte/store";

import * as api from "./api";
import * as error from "./error";
import * as event from "./event";
import * as identity from "./identity";
import * as notification from "./notification";
import * as remote from "./remote";
import { Appearance, CoCo, Settings } from "./settings";

// TYPES

export interface Session {
  identity?: identity.Identity;
  settings: Settings;
}

// STATE
const sessionStore = remote.createStore<Session>();
export const session = sessionStore.readable;

export const settings: Readable<Settings | null> = derived(
  sessionStore,
  sess => {
    if (sess.status === remote.Status.Success) {
      return sess.data.settings;
    }
    return null;
  }
);

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

const fetchSession = (): Promise<void> =>
  api
    .get<Session>(`session`)
    .then(sessionStore.success)
    .catch(sessionStore.error);

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

export const parseSeedsInput = (input: string): string[] => {
  return input
    .replace(/\r\n|\n|\r|\s/gm, ",")
    .split(",")
    .filter(seed => seed !== "");
};
