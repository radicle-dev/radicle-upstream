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

  return new Request(
      `http://localhost:8080/v1/${endpoint}`,
      {
        headers: {
          "Content-Type": "application/json"
        },
        ...init,
    });
}

async function http<T>(req: RequestInfo): Promise<T> {
  const res = await fetch(req);
  const body = await res.json();

  if (!res.ok) {
    throw body;
  }

  return body;
}

export async function del(endpoint: string, options?: Options): Promise<void> {
  return http<void>(request(endpoint, { method: "DELETE", ...options }));
}

export async function get<T>(endpoint: string, options?: Options): Promise<T> {
  return http<T>(request(endpoint, { method: "GET", ...options }));
}

export async function post<I, D>(endpoint: string, body: I, options?: Options): Promise<D> {
  return http<D>(request(endpoint, {
    method: "POST",
    body: JSON.stringify(body),
    ...options,
  }));
}
