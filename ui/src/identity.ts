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

interface Create {
  handle: string;
  displayName?: string;
  avatarUrl?: string;
}

interface Fetch {
  id: string;
}

type Msg = Create | Fetch;

interface CreateInput {
  handle: string;
  displayName?: string;
  avatarUrl?: string;
}

function update(event: event.Event<Kind, Msg>): void {
  switch (event.kind) {
    case Kind.Create:
      identityStore.loading();
      api.post<CreateInput, Identity>("identities", {
        handle: event.msg!.handle,
        displayName: event.msg!.displayName,
        avatarUrl: event.msg!.avatarUrl
      })
        .then(identityStore.success)
        .catch(identityStore.error)

      break;
    case Kind.Fetch:
      identityStore.loading();
      api.get<Identity>(`identities/${event.msg!.id}`)
        .then(identityStore.success)
        .catch(identityStore.error)

      break;
  }
}

export const create = event.create<Kind, Msg>(Kind.Create, update) 
