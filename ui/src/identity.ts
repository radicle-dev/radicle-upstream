import persistentStore from "svelte-persistent-store/dist";

import * as api from "./api";
import * as remote from "./remote";
import type { Urn } from "./urn";

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
  urn: Urn;
}

// Stub storing the user's registered Ethereum address
// TODO(nuno): read this value from the Person's document once implemented
export const ethereumAddress = persistentStore.local.writable<string | null>(
  "radicle.identity.ethereum.address.v0",
  null
);

// STATE
const creationStore = remote.createStore<Identity>();
export const store = creationStore.readable;

interface CreateInput {
  handle: string;
  passphrase: string;
}

export const createIdentity = (input: CreateInput): Promise<Identity> => {
  return api.post<CreateInput, Identity>("identities", input);
};

export const fetch = (urn: string): Promise<Identity> => {
  return api.get<Identity>(`identities/${urn}`);
};

// MOCK
export const fallback: Identity = {
  avatarFallback: {
    background: {
      r: 122,
      g: 112,
      b: 90,
    },
    emoji: "💡",
  },
  metadata: {
    handle: "cloudhead",
  },
  peerId: "hwd1yreyza9z77xzp1rwyxw9uk4kdrrzag5uybd7w1ihke18xxhxn6qu4oy",
  shareableEntityIdentifier:
    "rad:git:hwd1yreyza9z77xzp1rwyxw9uk4kdrrzag5uybd7w1ihke18xxhxn6qu4oy",
  urn: "rad:git:hwd1yreyza9z77xzp1rwyxw9uk4kdrrzag5uybd7w1ihke18xxhxn6qu4oy",
};
