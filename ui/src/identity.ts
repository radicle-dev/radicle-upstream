import * as api from "./api";
import * as event from "./event";
import * as remote from "./remote";

// TYPES
export interface Avatar {
  background: {
    r: number;
    g: number;
    b: number;
  };
  emoji: string;
}

export interface Identity {
  id: string;
  metadata: {
    handle: string;
  };
  registered?: string;
  avatarFallback: Avatar;
}

// STATE
const creationStore = remote.createStore<Identity>();
export const store = creationStore.readable;

const identityStore = remote.createStore<Identity>();
export const identity = identityStore.readable;

// EVENTS
enum Kind {
  Create = "CREATE",
  Fetch = "FETCH",
}

interface Create extends event.Event<Kind> {
  kind: Kind.Create;
  handle: string;
}

interface Fetch extends event.Event<Kind> {
  kind: Kind.Fetch;
  urn: string;
}

type Msg = Create | Fetch;

interface CreateInput {
  handle: string;
}

export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);

function update(msg: Msg): void {
  switch (msg.kind) {
    case Kind.Create:
      creationStore.loading();
      api
        .post<CreateInput, Identity>("identities", {
          handle: msg.handle,
        })
        .then(id => {
          creationStore.success(id);
        })
        .catch(creationStore.error);
      break;
    case Kind.Fetch:
      identityStore.loading();
      api
        .get<Identity>(`identities/${msg.urn}`)
        .then(identityStore.success)
        .catch(identityStore.error);
      break;
  }
}

export const create = event.create<Kind, Msg>(Kind.Create, update);

// MOCK
export const fallback = {
  id: "cloudhead@123abcd.git",
  metadata: {
    handle: "cloudhead",
  },
  avatarFallback: {
    background: {
      r: 122,
      g: 112,
      b: 90,
    },
    emoji: "💡",
  },
};
