// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";
import type { Fetcher, RequestOptions } from "./fetcher";

export interface Metadata {
  handle: string;
  ethereum: Ethereum | null;
}

const metadataSchema: zod.Schema<Metadata> = zod.object({
  handle: zod.string(),
  ethereum: zod
    .object({
      address: zod.string(),
      expiration: zod.string(),
    })
    .nullable(),
});

export interface Identity {
  urn: string;
  peerId: string;
  metadata: Metadata;
}

export const identitySchema: zod.Schema<Identity> = zod.object({
  urn: zod.string(),
  peerId: zod.string(),
  metadata: metadataSchema,
});

// A claim over an Ethereum Address
export interface Ethereum {
  address: string;
  expiration: string;
}

export interface RemoteIdentity {
  urn: string;
  peerIds: string[];
  metadata: Metadata;
}

export const remoteIdentitySchema = zod.object({
  urn: zod.string(),
  peerIds: zod.array(zod.string()),
  metadata: metadataSchema,
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
