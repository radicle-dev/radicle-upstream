import type * as zod from "zod";

// This module provides low-level capabilities to interact with a typed
// JSON HTTP API.

// Error that is thrown by `Fetcher` methods.
export class ResponseError extends Error {
  public response: Response;
  public body: unknown;
  constructor(response: Response, body_: unknown) {
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

    this.body = body_;
    this.response = response;
  }
}

export interface RequestOptions {
  abort?: AbortSignal;
}

export interface FetchParams {
  method: Method;
  // Path to append to the `Fetcher`s base URL to get the final uRL
  path: string;
  // Object that is serialized into JSON and sent as the data
  body?: unknown;
  options?: RequestOptions;
}

type Method = "GET" | "POST" | "PUT" | "DELETE";

export class Fetcher {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  // Execute a fetch and parse the result with the provided schema.
  // Return the parsed payload.
  //
  // Throws `ResponseError` if the response status code is not `200`.
  async fetchOk<T>(params: FetchParams, schema: zod.Schema<T>): Promise<T> {
    const response = await this.fetch(params);

    const responseBody = await response.json();

    if (!response.ok) {
      throw new ResponseError(response, responseBody);
    }

    return schema.parse(responseBody);
  }

  // Execute a fetch and ignore the response body.
  //
  // Throws `ResponseError` if the response status code is not `200`.
  async fetchOkNoContent(params: FetchParams): Promise<void> {
    const response = await this.fetch(params);

    if (!response.ok) {
      let responseBody = await response.text();
      try {
        responseBody = JSON.parse(responseBody);
      } catch (_e) {
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
  }: FetchParams): Promise<Response> {
    const headers: Record<string, string> = {};
    if (body !== undefined) {
      headers["content-type"] = "application/json";
    }
    return fetch(`${this.baseUrl}/v1/${path}`, {
      method,
      headers,
      body: body === undefined ? null : JSON.stringify(body),
      credentials: "include",
      ...options,
    });
  }
}
