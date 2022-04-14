// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as error from "./error";
import * as notification from "./notification";
import * as proxy from "./proxy";
import * as session from "./session";
import type { Ethereum, Identity, Metadata } from "proxy-client/identity";

export type { Identity, Metadata };

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
    await proxy.client.identity.update({
      ...metadata,
      ethereum,
    });
  } catch (err: unknown) {
    notification.showException(
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

// MOCK
export const fallback: Identity = {
  metadata: {
    handle: "cloudhead",
    ethereum: null,
  },
  peerId: "hwd1yreyza9z77xzp1rwyxw9uk4kdrrzag5uybd7w1ihke18xxhxn6qu4oy",
  urn: "rad:git:hwd1yreyza9z77xzp1rwyxw9uk4kdrrzag5uybd7w1ihke18xxhxn6qu4oy",
};
