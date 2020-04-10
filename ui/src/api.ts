const request = (endpoint: string, init?: object): Request => {
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

export async function get<T>(endpoint: string): Promise<T> {
  return http<T>(request(endpoint, { method: "GET" }));
}

export async function post<I, D>(endpoint: string, body: I): Promise<D> {
  return http<D>(request(endpoint, {
    method: "POST",
    body: JSON.stringify(body)
  }));
}
