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

function buildRequest<T, P>(
  endpoint: string | ((parameters: P) => string),
  method: string = "GET"
) {
  return async (params: P): Promise<T> => {
    let url: string;
    if (typeof endpoint == "string") {
      url = endpoint;
    } else if (typeof endpoint == "function") {
      url = endpoint(params);
    }
    const resp = await fetch(url, {
      headers: {
        Authorization: "Bearer test",
      },
      method,
    });

    const json = (await resp.json()) as APIResult<T>;

    if (!json.ok) {
      throw json.err;
    }

    return json.data;
  };
}

export const getLinks = buildRequest<Link[], {}>(
  "http://localhost:8000/api/link"
);

export const deleteLink = buildRequest<{}, { name: string }>(
  ({ name }) => `http://localhost:8000/api/link/${encodeURIComponent(name)}`,
  "DELETE"
);

// export const testAuth = async (): Promise<boolean> => {};
