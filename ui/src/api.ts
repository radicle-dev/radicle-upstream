async function http<T>(req: RequestInfo): Promise<T> {
  const res = await fetch(req);
  const body = await res.json();

  if (!res.ok) {
    throw body;
  }

  return body;
}

export async function post<T>(endpoint: string, body: any): Promise<T> {
  return http<T>(new Request(
    `http://localhost:8081/v1/${endpoint}`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(body)
  }));
}
