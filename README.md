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

Configure the server through environment variables or `Rocket.toml`; check [this guide](https://rocket.rs/v0.4/guide/configuration/) for more information. shorty runs on port `3000` by default.

## 📡 API

Use shorty's API to add/remove links.

Pass `POST` options as form parameters; e.g. `curl http://localhost:3000/api/link/gh -d "url=https://github.com/cjdenio&param=value"`

### `POST /api/link/<name>` - create or update a named link
Options:
- `url` - the url the redirect to

### `DELETE /api/link/<name>` - delete a named link
example: `curl -X DELETE http://localhost:3000/api/link/gh`