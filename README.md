# shorty

> High-performance link shortener written in Rust

## 💾 Hosting

In addition to being easy to build from source, shorty is available as a Docker image on [Docker Hub](https://hub.docker.com/r/cjdenio/shorty).

### 🏁 Prerequisites

- Some sort of Docker or Rust-compatible hosting; [clb.li](https://clb.li) runs on [CapRover](https://caprover.com)
- A persistent Redis database

### 🌎 Environment variables

- `DB_URL` - a valid Redis URL, e.g. redis://localhost:1234
- `TOKEN` - your desired API token; only required if you're using the API (described below).

### ⚙️ Server configuration

Configure the server through environment variables or `Rocket.toml`; check [this guide](https://rocket.rs/v0.4/guide/configuration/) for more information. shorty runs on port `8000` by default.

## 📡 API

Use shorty's API to add/remove links.

### Authentication

Provide your `TOKEN` (described above) as a bearer token, so set the `Authorization` header to `Bearer <token>`. Example: `curl -H "Authorization: Bearer token1234" http://localhost:8000/api/example/route`

### Requests

`POST` requests must all contain JSON payloads. `x-www-form-urlencoded` is _not_ supported.

### Responses

All responses are JSON, and follow this rough schema:

```jsonc
{
  // False if something went wrong
  "ok": true,

  // Will be a string if something went wrong
  "err": null,

  "data": {
    // Response data here...
  }
}
```

---

### 📋 `GET /api/link` - list all links

This method has no options.

---

### ➕ `POST /api/link` - create or update a named link

Options:

- `url` (string, **required**) - The URL the redirect to.
- `name` (string, optional) - The link's name. Leave blank to randomly generate a 10-character ID.
- `public` (bool, optional) - Whether or not to display this link on the public links page; **coming soon**

ℹ️ Note:
**`/` is a special value for `<name>` that creates a redirect for the root URL.**

---

### ❌ `DELETE /api/link/<name>` - delete a named link

example: `curl -X DELETE http://localhost:8000/api/link/gh`

ℹ️ Note:
**Please URL encode the `<name>` parameter if necessary; `/` will become `%2F`**

---

## 🚗 Roadmap

- UI
- Redirect code configuration
