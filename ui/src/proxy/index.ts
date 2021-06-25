// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";
import * as config from "../config";

import * as settings from "./settings";
import * as identity from "./identity";
import * as control from "./control";
import * as project from "./project";
import * as source from "./source";
import { Fetcher, ResponseError, RequestOptions } from "./fetcher";

export { ResponseError };

export interface Session {
  identity: identity.Identity;
  settings: settings.Settings;
}

export const sessionSchema: zod.ZodSchema<Session> = zod.object({
  identity: identity.identitySchema,
  settings: settings.settingsSchema,
});

export interface IdentityCreateParams {
  handle: string;
}

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

  constructor(baseUrl: string) {
    this.fetcher = new Fetcher(baseUrl);
    this.control = new control.Control(this.fetcher);
    this.project = new project.Client(this.fetcher);
    this.source = new source.Client(this.fetcher);
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

  async sessionSettingsSet(
    settings: settings.Settings,
    options?: RequestOptions
  ): Promise<void> {
    return this.fetcher.fetchOkNoContent({
      method: "POST",
      path: "session/settings",
      body: settings,
      options,
    });
  }

  async identityCreate(
    params: IdentityCreateParams,
    options?: RequestOptions
  ): Promise<identity.Identity> {
    return this.fetcher.fetchOk(
      {
        method: "POST",
        path: "identities",
        body: params,
        options,
      },
      identity.identitySchema
    );
  }

  async identityGet(
    urn: string,
    options?: RequestOptions
  ): Promise<identity.Identity> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `identities/${urn}`,
        options,
      },
      identity.identitySchema
    );
  }

  async remoteIdentityGet(
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

  async identityUpdate(
    params: identity.Metadata,
    options?: RequestOptions
  ): Promise<identity.Identity> {
    return this.fetcher.fetchOk(
      {
        method: "PUT",
        path: "identities",
        body: params,
        options,
      },
      identity.identitySchema
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
}

export const client = new Client(`http://${config.proxyAddress}`);
