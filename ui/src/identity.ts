// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as error from "./error";
import * as remote from "./remote";
import * as proxy from "./proxy";
import * as session from "./session";
import type { Ethereum, Identity } from "./proxy/identity";

export type { Identity };

// FIXME(xla): Improve type safety of it, this is a placeholder to avoid using strings everywhere.
export type PeerId = string;

const creationStore = remote.createStore<Identity>();
export const store = creationStore.readable;

export const createIdentity = (
  params: proxy.IdentityCreateParams
): Promise<Identity> => {
  return proxy.client.identityCreate(params);
};

// Claim the ownership of an Ethereum address, stored on the user's Radicle Identity.
export const claimEthAddress = async (address: string): Promise<void> =>
  updateEthereumClaim({
    address,
    expiration: getExpirationDate().toISOString(),
  });

// Remove, if present, a claim over an Ethereum address from the user's Radicle Identity.
export const removeEthClaim = async (): Promise<void> =>
  updateEthereumClaim(null);

const updateEthereumClaim = async (
  ethereum: Ethereum | null
): Promise<void> => {
  try {
    const unsealed = session.unsealed();
    if (!unsealed) {
      throw new Error("Session is not unsealed");
    }
    const { metadata } = unsealed.identity;
    await proxy.client.identityUpdate({
      ...metadata,
      ethereum,
    });
  } catch (err: unknown) {
    error.show(
      new error.Error({
        code: error.Code.UpdateEthereumClaimFailure,
        message: `Failed to update the Ethereum claim in your identity`,
        source: err,
      })
    );
    return;
  }

  await session.fetch();
};

function getExpirationDate(): Date {
  const days = 60;
  const result = new Date();
  result.setDate(result.getDate() + days);
  return result;
}

export const fetch = (urn: string): Promise<Identity> => {
  return proxy.client.identityGet(urn);
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
  urn: "rad:git:hwd1yreyza9z77xzp1rwyxw9uk4kdrrzag5uybd7w1ihke18xxhxn6qu4oy",
};
