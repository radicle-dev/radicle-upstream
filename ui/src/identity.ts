import { emit } from "./event";
import * as message from "./message";
import { createRemoteDataStore } from "./RemoteDataStore";

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
};

const identityStore = createRemoteDataStore<Identity>();

export const identity = identityStore.readable;

export enum Kind {
  Create = "CREATE",
  Created = "CREATED",
  Fetch = "FETCH",
  Fetched = "FETCHED",
};

interface Message {
  kind: Kind,
};

interface Create extends Message {
  kind: Kind.Create;
  handle: string;
  displayName?: string;
  avatarUrl?: string;
};

interface Created extends Message {
  kind: Kind.Created;
  identity: Identity;
};

interface Fetch extends Message {
  kind: Kind.Fetch;
  id: string;
};

interface Fetched extends Message {
  kind: Kind.Fetched;
  identity: Identity;
};

export type Msg = Create | Created | Fetch | Fetched;

export function update(msg: Msg): void {
  switch (msg.kind) {
    case Kind.Create:
      Api.create(msg.handle, msg.displayName, msg.avatarUrl);
      identityStore.loading();
      break;
    case Kind.Created:
      identityStore.success(msg.identity);
      break;
    case Kind.Fetch:
      Api.get(msg.id);
      identityStore.loading();
      break;
    case Kind.Fetched:
      identityStore.success(msg.identity);
      break;
  }
};

namespace Api {
  export function create(handle: string, displayName?: string, avatarUrl?: string): void {
    fetch("http://localhost:8080/v1/identities", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({ handle, displayName, avatarUrl })
    })
      .then(res => res.json())
      .then((data: Identity) => {
        emit({
          kind: message.Kind.Identity,
          msg: {
            kind: Kind.Created,
            identity: data,
          },
        })
      })
  }

  export function get(id: string): void {
    fetch(`http://localhost:8080/v1/identities/${id}`, { method: "GET" })
      .then(res => res.json())
      .then((data: Identity) => {
        emit({
          kind: message.Kind.Identity,
          msg: {
            kind: Kind.Fetched,
            identity: data,
          },
        })
      })
  }
}
