import * as api from "./api";
import * as event from "./event";
import * as remote from "./remote";
import * as urn from "./urn";

// TYPES
// FIXME(xla): Improve type safety of it, this is a placeholder to avoid using strings everywhere.
export type PeerId = string;

export interface Avatar {
  background: {
    r: number;
    g: number;
    b: number;
  };
  emoji: string;
}

export interface Identity {
  avatarFallback: Avatar;
  metadata: {
    handle: string;
  };
  peerId: PeerId;
  shareableEntityIdentifier: string;
  // FIXME(xla): Properly type urns.
  urn: urn.Urn;
}

// STATE
const creationStore = remote.createStore<Identity>();
export const store = creationStore.readable;

const identityStore = remote.createStore<Identity>();
export const identity = identityStore.readable;

// EVENTS
enum Kind {
  Fetch = "FETCH",
}

interface Fetch extends event.Event<Kind> {
  kind: Kind.Fetch;
  urn: string;
}

type Msg = Fetch;

interface CreateInput {
  handle: string;
  passphrase: string;
}

export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);

function update(msg: Msg): void {
  switch (msg.kind) {
    case Kind.Fetch:
      identityStore.loading();
      api
        .get<Identity>(`identities/${msg.urn}`)
        .then(identityStore.success)
        .catch(identityStore.error);
      break;
  }
}

export const createIdentity = (input: CreateInput): Promise<Identity> => {
  return api.post<CreateInput, Identity>("identities", input);
};

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
