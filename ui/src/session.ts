import * as api from "./api";
import * as event from "./event";
import * as identity from "./identity";
import * as org from "./org";
import * as remote from "./remote";
import * as settings from "./settings";
import * as transaction from "./transaction";

// TYPES

export interface Session {
  identity?: identity.Identity;
  orgs: org.Org[];
  settings: settings.Settings;
}

// STATE
const sessionStore = remote.createStore<Session>();
export const session = sessionStore.readable;

// EVENTS
enum Kind {
  Clear = "CLEAR",
  ClearCache = "CLEAR_CACHE",
  Fetch = "FETCH",
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

type Msg = Clear | ClearCache | Fetch;

const fetchSession = (): Promise<void> =>
      api.get<Session>(`session`)
        .then(sessionStore.success)
        .catch(sessionStore.error)

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.Clear:
      api.del(`session`)
        .then(fetchSession)

      break;

    case Kind.ClearCache:
      api.del(`session/cache`)
        .then(() => transaction.fetchList())

      break;

    case Kind.Fetch:
      sessionStore.loading();
      fetchSession();

      break;
  }
}

export const clear = event.create<Kind, Msg>(Kind.Clear, update);
export const clearCache = event.create<Kind, Msg>(Kind.ClearCache, update);
export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
