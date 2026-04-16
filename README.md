# Settings Service

The service stores **secrets** and **YAML configuration templates**. Clients request a compiled template at `/settings/{product}/{template}` — the service finds the template, substitutes `${secret-id}` placeholders with secret values, and returns the result.

## Remote Secret

**Problem.** The same secret may need to resolve to two different values depending on where the client connects from:

- **Local** — client is on the same network (VPN / internal perimeter). Needs the internal address, e.g. `db.internal:5432`.
- **Remote** — client connects over the internet. Needs the public address of the same resource, e.g. `db.example.com:5432`.

Instead of maintaining two separate secrets or two templates, each secret carries an optional second value — `remote_value`.

**Model.** `SecretItem` (see [settings-service/src/models/secret_item.rs](settings-service/src/models/secret_item.rs)):

```rust
pub struct SecretItem {
    pub id: String,
    pub content: Content,                  // primary value (local)
    pub remote_value: Option<Content>,     // value used for remote clients
    // ...
}
```

**Local vs remote detection.** The client sends an HTTP header `env-info` with its environment name (e.g. `AZURE-UK-PROD`, `AWS-US-STAGE`). The service compares it against the configured `local_env_prefixes` list:

- value starts with any of the prefixes → **local** → `content` is used
- otherwise → **remote** → `remote_value` is used (if set and non-empty); falls back to `content`
- header is absent → **local** (backward compatibility with older clients)
- `local_env_prefixes` is not configured → always **local**

Logic lives in [`SettingsModel::is_local_env`](settings-service/src/settings.rs) and [`SecretItem::resolve_content`](settings-service/src/models/secret_item.rs).

**Where it applies.** In the template-rendering middleware [settings-service/src/http_server/settings_middleware.rs](settings-service/src/http_server/settings_middleware.rs): it reads the `env-info` header, computes `is_remote`, and passes the flag into `populate_secrets` when substituting placeholders.

## Configuration

Put settings file `~/.settings-service` with content:

```yaml
SettingsService:
  http_port: 80
  grpc_port: 8888
  encryption_key: xxxxxxxxxx
  env: demo
  favicon_color: yellow
  max_level_of_secrets_to_export: 0
  data_path: ~/settings-service-data

  # Prefixes of "local" environments. Used when rendering
  # /settings/{product}/{template}: if the client sends the `env-info`
  # header and its value starts with one of these prefixes, the
  # environment is treated as local (the secret's `value` is used).
  # Otherwise it is remote (the secret's `remote_value` is used, with
  # fallback to `value`). If the header is absent or the list is not
  # configured, the environment is treated as local.
  local_env_prefixes:
    - "AZURE-UK-"
    - "AZURE-EU-"
    - "ON-PREM-"
```

## Behavior example

Secret `db-password` has `value = local-pass`, `remote_value = remote-pass`.

| `env-info` header | Result |
|---|---|
| absent | `local-pass` |
| `AZURE-UK-DEV` | `local-pass` (prefix matched) |
| `AZURE-EU-STAGE` | `local-pass` (prefix matched) |
| `AZURE-US-PROD` | `remote-pass` (no prefix match) |
| `AWS-PROD` | `remote-pass` |
| `AZURE-US-PROD` and `remote_value` not set | `local-pass` (fallback) |
