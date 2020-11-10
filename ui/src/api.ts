import * as qs from "qs";

interface Options {
  query?: Record<string, unknown>;
}

interface Init extends Options {
  body?: string;
  method: string;
  headers?: Record<string, string>;
  query?: Record<string, unknown>;
}

export class ResponseError extends Error {
  public response;
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
    this.response = response;
  }
}

const request = (endpoint: string, init?: Init): Request => {
  if (init !== undefined && init.query !== undefined) {
    endpoint = `${endpoint}?${qs.stringify(init.query)}`;
  }

  return new Request(`http://localhost:17246/v1/${endpoint}`, {
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
    ...init,
  });
};

const http = async <T>(req: RequestInfo): Promise<T> => {
  const res = await fetch(req);
  const body = await res.json();

  // For non-success status codes we throw the body as it carries the error type.
  if (!res.ok) {
    throw new ResponseError(res, body);
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
  options?: Options
): Promise<D> =>
  http<D>(
    request(endpoint, {
      method: "POST",
      body: JSON.stringify(body),
      ...options,
    })
  );

export const put = async <I, D>(
  endpoint: string,
  body: I,
  options?: Options
): Promise<D> =>
  http<D>(
    request(endpoint, {
      method: "PUT",
      body: body !== null ? JSON.stringify(body) : undefined,
      ...options,
    })
  );

export const set = async <T>(
  endpoint: string,
  body: T,
  options?: Options
): Promise<null> =>
  noContent(
    request(endpoint, {
      method: "POST",
      body: JSON.stringify(body),
      ...options,
    })
  );

const delay = (delay: number) => {
  return new Promise((resolve, _reject) => {
    setTimeout(resolve, delay);
  });
};

export const withRetry = async <T>(
  request: () => Promise<T>,
  delayTime: number
): Promise<T> => {
  for (let retries = 0; ; retries++) {
    try {
      return await request();
    } catch (error) {
      if (error.message !== "Failed to fetch" || retries > 200) {
        throw error;
      }
    }
    await delay(delayTime);
  }
};
