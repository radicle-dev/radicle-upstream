import * as zod from "zod";
import * as config from "../config";

import * as settings from "./settings";
import * as identity from "./identity";
import * as control from "./control";
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

  constructor(baseUrl: string) {
    this.fetcher = new Fetcher(baseUrl);
    this.control = new control.Control(this.fetcher);
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

export const withRetry = async <T>(
  request: () => Promise<T>,
  delayTime: number,
  retries: number
): Promise<T> => {
  for (; ; retries--) {
    try {
      return await request();
    } catch (error) {
      if (error.message !== "Failed to fetch" || retries < 0) {
        throw error;
      }
    }
    await sleep(delayTime);
  }
};

const sleep = (delay: number) => {
  return new Promise((resolve, _reject) => {
    setTimeout(resolve, delay);
  });
};
