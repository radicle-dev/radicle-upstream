// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";
import type { Fetcher, RequestOptions } from "./fetcher";

export interface RemoteIdentity {
  metadata: Metadata;
  urn: string;
  peerIds: string[];
}

export interface Identity {
  metadata: Metadata;
  urn: string;
  peerId: string;
}

export const identitySchema = zod.object({
  metadata: zod.object({
    handle: zod.string(),
    ethereum: zod
      .object({
        address: zod.string(),
        expiration: zod.string(),
      })
      .nullable(),
  }),
  urn: zod.string(),
  peerId: zod.string(),
});

export interface Metadata {
  handle: string;
  ethereum: Ethereum | null;
}

// A claim over an Ethereum Address
export interface Ethereum {
  address: string;
  expiration: string;
}

export const remoteIdentitySchema = zod.object({
  metadata: zod.object({
    handle: zod.string(),
    ethereum: zod
      .object({
        address: zod.string(),
        expiration: zod.string(),
      })
      .nullable(),
  }),
  urn: zod.string(),
  peerIds: zod.array(zod.string()),
});

export class Client {
  private fetcher: Fetcher;

  constructor(fetcher: Fetcher) {
    this.fetcher = fetcher;
  }

  async create(
    params: { handle: string },
    options?: RequestOptions
  ): Promise<Identity> {
    return this.fetcher.fetchOk(
      {
        method: "POST",
        path: "identities",
        body: params,
        options,
      },
      identitySchema
    );
  }

  async update(params: Metadata, options?: RequestOptions): Promise<Identity> {
    return this.fetcher.fetchOk(
      {
        method: "PUT",
        path: "identities",
        body: params,
        options,
      },
      identitySchema
    );
  }
}
