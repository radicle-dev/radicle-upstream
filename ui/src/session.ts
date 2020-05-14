import * as api from "./api";
import * as event from "./event";
import * as identity from "./identity";
import * as org from "./org";
import * as remote from "./remote";
import * as settings from "./settings";

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
  Fetch = "FETCH",
}

interface Clear extends event.Event<Kind> {
  kind: Kind.Clear;
}

interface Fetch extends event.Event<Kind> {
  kind: Kind.Fetch;
}

type Msg = Clear | Fetch;

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
    case Kind.Fetch:
      sessionStore.loading();
      fetchSession();

      break;
  }
}

export const clear = event.create<Kind, Msg>(Kind.Clear, update);
export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
