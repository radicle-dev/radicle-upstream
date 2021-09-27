// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";

import { config } from "ui/src/config";

import * as identity from "./identity";
import * as control from "./control";
import * as project from "./project";
import * as source from "./source";
import { Fetcher, ResponseError, RequestOptions } from "./fetcher";

export { ResponseError };

export interface Session {
  identity: identity.Identity;
}

export const sessionSchema: zod.ZodSchema<Session> = zod.object({
  identity: identity.identitySchema,
});

interface KeyStoreUnsealParams {
  passphrase: string;
}

interface KeyStoreCreateParams {
  passphrase: string;
}

export class Client {
  private fetcher: Fetcher;

  public control: control.Control;
  public project: project.Client;
  public source: source.Client;
  public identity: identity.Client;

  constructor(baseUrl: string) {
    this.fetcher = new Fetcher(baseUrl);
    this.control = new control.Control(this.fetcher);
    this.project = new project.Client(this.fetcher);
    this.source = new source.Client(this.fetcher);
    this.identity = new identity.Client(this.fetcher);
  }

  async sessionGet(options?: RequestOptions): Promise<Session> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: "session",
        options,
      },
      sessionSchema
    );
  }

  async personGet(
    urn: string,
    options?: RequestOptions
  ): Promise<identity.RemoteIdentity> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `identities/remote/${urn}`,
        options,
      },
      identity.remoteIdentitySchema
    );
  }

  async keyStoreUnseal(
    params: KeyStoreUnsealParams,
    options?: RequestOptions
  ): Promise<void> {
    await this.fetcher.fetchOkNoContent({
      method: "POST",
      path: "keystore/unseal",
      body: params,
      options,
    });
  }

  async keyStoreCreate(
    params: KeyStoreCreateParams,
    options?: RequestOptions
  ): Promise<void> {
    return this.fetcher.fetchOkNoContent({
      method: "POST",
      path: "keystore",
      body: params,
      options,
    });
  }

  async seedsGet(options?: RequestOptions): Promise<string[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: "session/seeds",
        options,
      },
      zod.array(zod.string())
    );
  }

  async seedsPut(seeds: string[], options?: RequestOptions): Promise<void> {
    return this.fetcher.fetchOkNoContent({
      method: "PUT",
      path: "session/seeds",
      body: seeds,
      options,
    });
  }
}

export const client = new Client(`http://${config.proxyAddress}`);
