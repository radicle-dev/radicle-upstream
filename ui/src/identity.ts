import * as api from "./api";
import { createStore } from "./remote";

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

const identityStore = createStore<Identity>();

export const identity = identityStore.readable;

export enum Kind {
  Create = "CREATE",
  Created = "CREATED",
  Fetch = "FETCH",
  Fetched = "FETCHED",
}

interface Message {
  kind: Kind;
}

interface Create extends Message {
  kind: Kind.Create;
  handle: string;
  displayName?: string;
  avatarUrl?: string;
}

interface Fetch extends Message {
  kind: Kind.Fetch;
  id: string;
}

export type Msg = Create | Fetch;

interface CreateInput {
  handle: string;
  displayName?: string;
  avatarUrl?: string;
}

export function update(msg: Msg): void {
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
