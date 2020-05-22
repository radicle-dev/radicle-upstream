import { Readable, derived, get } from "svelte/store";

import * as api from "./api";
import * as error from "./error";
import * as event from "./event";
import * as identity from "./identity";
import * as notification from "./notification";
import * as org from "./org";
import * as remote from "./remote";
import { Appearance, Registry, Settings } from "./settings";
import * as transaction from "./transaction";

// TYPES

export interface Session {
  identity?: identity.Identity;
  orgs: org.Org[];
  settings: Settings;
}

// STATE
const sessionStore = remote.createStore<Session>();
export const session = sessionStore.readable;

export const settings: Readable<Settings | null> =
  derived(sessionStore, (sess) => {
    if (sess.status === remote.Status.Success) {
      return sess.data.settings;
    }
    return null;
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

const fetchSession = (): Promise<void> =>
  api.get<Session>(`session`)
    .then(sessionStore.success)
    .catch(sessionStore.error);

const updateSettings = (settings: Settings): Promise<void> =>
  api.set<Settings>(`session/settings`, settings)
    .then(fetchSession)
    .catch((err: error.Error) => notification.error(err.message));

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.Clear:
      api.del(`session`)
        .then(fetchSession)
        .then(() => transaction.fetchList());

      break;

    case Kind.ClearCache:
      api.del(`session/cache`)
        .then(() => transaction.fetchList());

      break;

    case Kind.Fetch:
      sessionStore.loading();
      fetchSession()
        .then(() => transaction.fetchList());

      break;

    case Kind.UpdateSettings:
      updateSettings(msg.settings);

      break;
  }
}

export const clear = event.create<Kind, Msg>(Kind.Clear, update);
export const clearCache = event.create<Kind, Msg>(Kind.ClearCache, update);
export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
export const updateAppearance = (appearance: Appearance): void =>
  event.create<Kind, Msg>(Kind.UpdateSettings, update)({
    settings: { ...get(settings), appearance }
  });
export const updateRegistry = (registry: Registry): void =>
  event.create<Kind, Msg>(Kind.UpdateSettings, update)({
    settings: { ...get(settings), registry }
  });
