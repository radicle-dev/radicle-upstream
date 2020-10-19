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
  shareableEntityIdentifier: string;
}

// STATE
const creationStore = remote.createStore<Identity>();
export const store = creationStore.readable;

const identityStore = remote.createStore<Identity>();
export const identity = identityStore.readable;

const contributorStore = remote.createStore<Identity>();
export const contributor = contributorStore.readable;

// EVENTS
enum Kind {
  Fetch = "FETCH",
  FetchContributor = "FETCH_CONTRIBUTOR",
}

interface Fetch extends event.Event<Kind> {
  kind: Kind.Fetch;
  urn: string;
}

interface FetchContributor extends event.Event<Kind> {
  kind: Kind.FetchContributor;
  peerId: string;
  urn: string;
}

type Msg = Fetch | FetchContributor;

interface CreateInput {
  handle: string;
  passphrase: string;
}

export const fetch = event.create<Kind, Msg>(Kind.Fetch, update);
export const fetchContributor = event.create<Kind, Msg>(
  Kind.FetchContributor,
  update
);

function update(msg: Msg): void {
  switch (msg.kind) {
    case Kind.Fetch:
      identityStore.loading();
      api
        .get<Identity>(`identities/${msg.urn}`)
        .then(identityStore.success)
        .catch(identityStore.error);
      break;

    case Kind.FetchContributor:
      contributorStore.loading();
      api
        .get<Identity>(`identities/${msg.urn}/${msg.peerId}`)
        .then(contributorStore.success)
        .catch(contributorStore.error);

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
