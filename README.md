# shorty

🔗 High-performance link shortener written in Rust

> ### ⚠️ WARNING ⚠️
>
> Redis-backed installations _are not longer supported_. To switch to Postgres, please see the [migration guide](MIGRATION.md).

## 💾 Hosting

In addition to being easy to build from source, shorty is available as a Docker image on [Docker Hub](https://hub.docker.com/r/cjdenio/shorty).

### 🏁 Prerequisites

- Some sort of Docker or Rust-compatible hosting. [clb.li](https://clb.li) runs on [CapRover](https://caprover.com), for example, but [Heroku](https://heroku.com) is a good free option.
- A Postgres database (Heroku Postgres is your friend if your running on Heroku)

### 🌎 Environment variables

- `DATABASE_URL` - a valid Postgres URL, e.g. postgres://user:password@localhost:5432/database _(automatically set when using Heroku Postgres)_
- `TOKEN` - your desired API token; only required if you're using the API (described below).
- `PORT` - Change the port the server listens on; defaults to `8000`

## 📡 API

You can use shorty's API to add/remove links. No UI is available quite yet, but soon!

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
