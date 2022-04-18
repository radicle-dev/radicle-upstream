// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";
import * as bacon from "baconjs";

import * as identity from "./identity";
import * as control from "./control";
import * as project from "./project";
import * as source from "./source";
import { events, Event, EventSourceConstructor } from "./events";
import { Fetcher, BaseFetch, ResponseError, RequestOptions } from "./fetcher";

export { RequestOptions, ResponseError, Event };

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

export interface Diagnostics {
  storage: {
    gitDirPath: string;
    refsTree: string[];
  };
  peer?: unknown;
}

export const diagnosticsSchema = zod.object({
  storage: zod.object({
    gitDirPath: zod.string(),
    refsTree: zod.array(zod.string()),
  }),
  peer: zod.unknown(),
});

export class ProxyClient {
  private fetcher: Fetcher;
  #baseUrl: string;
  #eventSource?: EventSourceConstructor;

  public control: control.Control;
  public project: project.Client;
  public source: source.Client;
  public identity: identity.Client;

  public constructor(
    baseUrl: string,
    fetch?: BaseFetch,
    eventSource?: EventSourceConstructor
  ) {
    this.#baseUrl = baseUrl;
    this.#eventSource = eventSource;
    this.fetcher = new Fetcher(baseUrl, fetch);
    this.control = new control.Control(this.fetcher);
    this.project = new project.Client(this.fetcher);
    this.source = new source.Client(this.fetcher);
    this.identity = new identity.Client(this.fetcher);
  }

  public events(): bacon.EventStream<Event> {
    let eventSource;
    if (this.#eventSource !== undefined) {
      eventSource = this.#eventSource;
    } else if (globalThis.EventSource === undefined) {
      throw new Error();
    } else {
      eventSource = globalThis.EventSource;
    }
    return events(eventSource, this.#baseUrl);
  }

  public async diagnosticsGet(options?: RequestOptions): Promise<Diagnostics> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: "diagnostics",
        options,
      },
      diagnosticsSchema
    );
  }

  public async sessionGet(options?: RequestOptions): Promise<Session> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: "session",
        options,
      },
      sessionSchema
    );
  }

  public async personGet(
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

  public async keyStoreUnseal(
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

  public async keyStoreCreate(
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
