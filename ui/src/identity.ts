import * as api from "./api";
import * as error from "./error";
import * as remote from "./remote";
import * as session from "./session";
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

export interface Metadata {
  handle: string;
  ethereum: Ethereum | null;
}

// A claim over an Ethereum Address
export interface Ethereum {
  // TODO(nuno): make type-safe?
  address: string;
  expiration: Date;
}

export interface Identity {
  avatarFallback: Avatar;
  metadata: Metadata;
  peerId: PeerId;
  shareableEntityIdentifier: string;
  urn: Urn;
}

// STATE
const creationStore = remote.createStore<Identity>();
export const store = creationStore.readable;

export const createIdentity = (metadata: Metadata): Promise<Identity> => {
  return api.post<Metadata, Identity>("identities", metadata);
};

// Claim the ownership of an Ethereum address, stored on the user's Radicle Identity.
export const claimEthAddress = async (address: string): Promise<void> =>
  updateEthereumClaim({ address, expiration: getExpirationDate() });

// Remove, if present, a claim over an Ethereum address from the user's Radicle Identity.
export const removeEthClaim = async (): Promise<void> =>
  updateEthereumClaim(null);

const updateEthereumClaim = async (
  ethereum: Ethereum | null
): Promise<void> => {
  try {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const { metadata } = session.unsealed()!.identity;
    await api.put<Metadata, void>("identities", {
      ...metadata,
      ethereum,
    });
  } catch (err) {
    error.show({
      code: error.Code.UpdateEthereumClaimFailure,
      message: `Failed to update the Ethereum claim in your identity: ${err.message}`,
      source: error.fromException(err),
    });
    return;
  }

  session.fetch();
};

function getExpirationDate() {
  const days = 60;
  const result = new Date(days);
  result.setDate(result.getDate() + days);
  return result;
}

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
    ethereum: null,
  },
  peerId: "hwd1yreyza9z77xzp1rwyxw9uk4kdrrzag5uybd7w1ihke18xxhxn6qu4oy",
  shareableEntityIdentifier:
    "rad:git:hwd1yreyza9z77xzp1rwyxw9uk4kdrrzag5uybd7w1ihke18xxhxn6qu4oy",
  urn: "rad:git:hwd1yreyza9z77xzp1rwyxw9uk4kdrrzag5uybd7w1ihke18xxhxn6qu4oy",
};
