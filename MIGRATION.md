# Redis > Postgres Migration Guide

Welcome! If you're reading this, you're probably running a legacy Redis-powered installation of shorty.

Don't worry, because migrating to Postgres is super easy; barely an inconvenience!

## Step 0: Update shorty

Before doing anything else, please update shorty to the latest commit. If running in Docker, pull the latest image from either `cjdenio/shorty:latest` or `ghcr.io/cjdenio/shorty:latest`.

## Step 1: Start up a Postgres database

That's it. You'll need to start a Postgres database before continuing migration.

## Step 2: Set environment variables

- `REDIS_URL` should be set to your Redis datastore's URL. Please copy this over from `DB_URL`.
- `DATABASE_URL` (or `DB_URL`, they're identical) should be set to the _Postgres database's URL_. You probably already have this set to your Redis datastore's URL, so please change this.
- `TOKEN` - if you haven't already set a token, please do that now. (more info in the README)

## Step 3: Migration!

To run the migration, simply run the following API request in a terminal:

```bash
curl "http://<YOUR INSTANCE'S URL>/api/migrate" -H "Authorization: Bearer <YOUR TOKEN>"
```

The response should contain something like `Successfully migrated 5 links from Redis to Postgres!`. If so, you're all set! 🎉

To verify that all links have been moved over, make a request to `/api/link`:

```bash
curl "http://<YOUR INSTANCE'S URL>/api/link" -H "Authorization: Bearer <YOUR TOKEN>"
```

The output should contain all of your links.

## Step 4: Clean up

You can now safely destroy your Redis database and unset the `REDIS_URL` env var.
