import * as api from "./api";
import * as event from "./event";
import * as remote from "./remote";

// Types.
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
    displayName?: string;
    avatarUrl?: string;
  };
  registered?: string;
  avatarFallback: Avatar;
}

const identityStore = remote.createStore<Identity>();
export const identity = identityStore.readable;

// Events.
enum Kind {
  Create = "CREATE",
  Fetch = "FETCH",
}

interface Create extends event.Event<Kind> {
  kind: Kind.Create;
  handle: string;
  displayName?: string;
  avatarUrl?: string;
}

interface Fetch extends event.Event<Kind> {
  kind: Kind.Fetch;
  id: string;
}

type Msg = Create | Fetch;

interface CreateInput {
  handle: string;
  displayName?: string;
  avatarUrl?: string;
}

function update(msg: Msg): void {
  switch (msg.kind) {
    case Kind.Create:
      identityStore.loading();
      api.post<CreateInput, Identity>("identities", {
        handle: msg.handle,
        displayName: msg.displayName,
        avatarUrl: msg.avatarUrl
      })
        .then(identityStore.success)
        .catch(identityStore.error)

      break;
    case Kind.Fetch:
      identityStore.loading();
      api.get<Identity>(`identities/${msg.id}`)
        .then(identityStore.success)
        .catch(identityStore.error)

      break;
  }
}

export const create = event.create<Kind, Msg>(Kind.Create, update) 
