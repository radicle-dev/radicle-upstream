// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type * as zod from "zod";
import qs from "qs";

// This module provides low-level capabilities to interact with a typed
// JSON HTTP API.

// Error that is thrown by `Fetcher` methods.
export class ResponseError extends Error {
  public response: RawResponse;
  public body: unknown;

  // The "variant" field of the response body, if present. This field
  // is present in all proxy API errors.
  public variant: string | undefined;

  public constructor(response: RawResponse, body_: unknown) {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const body: any = body_;
    if (
      typeof body === "object" &&
      body !== null &&
      typeof body.message === "string"
    ) {
      super(body.message);
    } else {
      super("Response error");
    }

    if (
      typeof body === "object" &&
      body !== null &&
      typeof body.variant === "string"
    ) {
      this.variant = body.variant;
    }

    this.body = body_;
    this.response = response;
  }
}

// Error that is thrown by `Fetcher` methods when parsing the response
// body fails.
export class ResponseParseError extends Error {
  public method: string;
  public path: string;
  public body: unknown;
  public zodIssues: zod.ZodIssue[];

  public constructor(
    method: string,
    path: string,
    body: unknown,
    zodIssues: zod.ZodIssue[]
  ) {
    super("Failed to parse response body");
    this.method = method;
    this.path = path;
    this.body = body;
    this.zodIssues = zodIssues;
  }
}

export interface RequestOptions {
  abort?: AbortSignal;
}

export interface FetchParams {
  method: Method;
  // Path to append to the `Fetcher`s base URL to get the final URL
  path: string;
  // Object that is serialized into JSON and sent as the data
  body?: unknown;
  // Query parameters to be serialized with `qs`.
  query?: Record<string, unknown>;
  options?: RequestOptions;
}

type Method = "GET" | "POST" | "PUT" | "DELETE";

interface RawResponse {
  readonly ok: boolean;
  readonly status: number;
  text(): Promise<string>;
  json(): Promise<unknown>;
}

// Signature of a `fetch` function that is shared by the WhatWG
// implemention and the `node-fetch` package implemention.
export type BaseFetch = (
  url: string,
  opts?: {
    method?: string;
    headers: Record<string, string>;
    body?: string;
    credentials?: "include";
    signal?: AbortSignal;
  }
) => Promise<RawResponse>;

export class Fetcher {
  private baseUrl: string;
  #fetch: BaseFetch;

  public constructor(baseUrl: string, fetch?: BaseFetch) {
    this.#fetch = fetch || globalThis.fetch;
    this.baseUrl = baseUrl;
  }

  // Execute a fetch and parse the result with the provided schema.
  // Return the parsed payload.
  //
  // Throws `ResponseError` if the response status code is not `200`.
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  public async fetchOk<T extends zod.ZodType<any>>(
    params: FetchParams,
    schema: T
  ): Promise<zod.infer<T>> {
    const response = await this.fetch(params);

    const responseBody = await response.json();

    if (!response.ok) {
      throw new ResponseError(response, responseBody);
    }

    const result = schema.safeParse(responseBody);
    if (result.success) {
      return result.data;
    } else {
      throw new ResponseParseError(
        params.method,
        params.path,
        responseBody,
        result.error.errors
      );
    }
  }

  // Execute a fetch and ignore the response body.
  //
  // Throws `ResponseError` if the response status code is not `200`.
  public async fetchOkNoContent(params: FetchParams): Promise<void> {
    const response = await this.fetch(params);

    if (!response.ok) {
      let responseBody = await response.text();
      try {
        responseBody = JSON.parse(responseBody);
      } catch (_e: unknown) {
        // We keep the original text response body
      }
      throw new ResponseError(response, responseBody);
    }
  }

  private async fetch({
    method,
    path,
    body,
    options = {},
    query,
  }: FetchParams): Promise<RawResponse> {
    const headers: Record<string, string> = {};
    if (body !== undefined) {
      headers["content-type"] = "application/json";
    }

    let url = `${this.baseUrl}/v1/${path}`;
    if (query) {
      url = `${url}?${qs.stringify(query)}`;
    }
    return this.#fetch.call(undefined, url, {
      method,
      headers,
      body: body === undefined ? undefined : JSON.stringify(body),
      credentials: "include",
      signal: options.abort,
    });
  }
}
