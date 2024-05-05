# workers-rs-axum-tursodb

Updated from the example
https://github.com/tursodatabase/examples/tree/master/app-todo-axum

And using / adapter with
https://github.com/tursodatabase/libsql-client-rs?tab=readme-ov-file#cloudflare-workers

Tested

# Axum Todo List

A todo list example featuring [Turso](https://turso.tech) and [Axum](https://github.com/tokio-rs/axum).

## Development

Create a turso database.

```sh
turso db create <db-name>
```

Get the database credentials:

```sh
# db url
turso db show --url <db-name>

# authentication token
turso db tokens create <db-name>
```

Store the credentials inside a `.dev.vars` file next to your wrangler.toml:

```text
LIBSQL_CLIENT_URL=
LIBSQL_CLIENT_TOKEN=
```

## Run project

```sh
npx wrangler dev
```

Add a new task:

```sh
curl "http://localhost:8787/todos" \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{"task": "Do task m"}'
```

Get the list of added tasks:

```sh
curl "http://localhost:8787/todos"
```
