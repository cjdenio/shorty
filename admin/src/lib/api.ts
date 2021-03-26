export interface APIResult<T> {
  ok: boolean;
  err: string | null;
  data: T;
}
export interface Link {
  name: string;
  url: string;
  public: boolean;
  description: string | null;
}

export async function getLinks(): Promise<Link[]> {
  const resp = await fetch("http://localhost:8000/api/link", {
    headers: {
      Authorization: "Bearer test",
    },
  });

  const json = (await resp.json()) as APIResult<Link[]>;

  if (!json.ok) {
    throw json.err;
  }

  return json.data;
}
