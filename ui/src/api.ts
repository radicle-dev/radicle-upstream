import * as qs from "qs";

interface Options {
  query?: object;
}

interface Init extends Options {
  body?: string;
  method: string;
  headers?: Record<string, string>;
  query?: object;
}

const request = (endpoint: string, init?: Init): Request => {
  if (init !== undefined && init.query !== undefined) {
    endpoint = `${endpoint}?${qs.stringify(init.query)}`;
  }

  return new Request(`http://localhost:8080/v1/${endpoint}`, {
    headers: {
      "Content-Type": "application/json",
    },
    ...init,
  });
};

const http = async <T>(req: RequestInfo): Promise<T> => {
  const res = await fetch(req);
  const body = await res.json();

  // For non-success status codes we throw the body as it carries the error type.
  if (!res.ok) {
    throw body;
  }

  return body;
};

const noContent = async (req: RequestInfo): Promise<null> => {
  const res = await fetch(req);

  if (res.status !== 204) {
    const body = await res.json();
    throw body;
  }

  return null;
};

export const del = async (endpoint: string, options?: Options): Promise<null> =>
  noContent(request(endpoint, { method: "DELETE", ...options }));

export const get = async <T>(endpoint: string, options?: Options): Promise<T> =>
  http<T>(request(endpoint, { method: "GET", ...options }));

export const post = async <I, D>(
  endpoint: string,
  body: I,
  options?: Options,
): Promise<D> =>
  http<D>(
    request(endpoint, {
      method: "POST",
      body: JSON.stringify(body),
      ...options,
    }),
  );

export const set = async <T>(
  endpoint: string,
  body: T,
  options?: Options,
): Promise<null> =>
  noContent(
    request(endpoint, {
      method: "POST",
      body: JSON.stringify(body),
      ...options,
    }),
  );
