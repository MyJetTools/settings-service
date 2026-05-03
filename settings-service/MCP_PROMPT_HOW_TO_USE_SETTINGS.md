SettingsService MCP server — guide for AI agents wiring up applications.

# What this service stores

SettingsService manages two kinds of entities, both scoped by a `product_id`:

1. **Templates** — YAML configuration documents. A template is identified by `(product_id, template_id)`. Its body is raw YAML text that may contain placeholders of the form `${secret_id}`. Templates are NOT validated as YAML on save; bad YAML surfaces only at compile time.

2. **Secrets** — named values referenced by templates. A secret is identified by `(product_id, secret_id)` and has:
   - `value` — the value used in the root (local) datacenter.
   - `remote_value` (optional) — an alternative value used when rendering for a non-root (remote) datacenter. If absent or empty, the root `value` is used in both cases.
   - `level` (u8) — permission level. A secret's value may itself contain `${other_secret}` placeholders; nested resolution is allowed only when the inner secret's `level` is high enough.
   - `description` (optional, free text) — human-readable purpose of the secret. Use it to find the right secret without guessing from the name.

# The "Shared" product

The literal `"Shared"` is a special product scope. Templates and secrets stored under `"Shared"` are visible from every product. When a template owned by product X references `${some_id}`, the lookup is "first try X's secrets, then fall back to Shared secrets".

When calling any tool that takes `product_id`, pass `"Shared"` (case-insensitive) to operate on the shared scope.

# How template compilation works

`compile_template_yaml` returns the template's YAML with each `${secret_id}` placeholder REWRITTEN — never substituted with the real secret value. The marker is:
- `SECRET_<id>_VALUE` — the secret exists in scope (resolvable from the product or from Shared).
- `SECRET_<id>_NOT_FOUND` — the secret does not exist; the template references something that has not been created.

This guarantees that real secret values are never returned by this tool — the AI sees the YAML structure and the names of every dependency, with missing ones standing out at a glance. The same response also lists the missing ids in `missing_keys` (deduplicated) plus a boolean `has_missing_keys`, so you can summarize "what is unresolved" without re-parsing the YAML.

Local vs remote distinction does NOT apply to this tool: since values are not substituted, the rendering is independent of datacenter. To learn whether a particular secret has a remote-datacenter variant, call `get_secret_info` and read its `has_remote_value` flag.

# Privacy rules baked into the tools

- `list_secrets` returns metadata only — id, description, level, `has_remote_value`. It NEVER returns the actual `value` or `remote_value`. To inspect a value, follow up with `get_secret_info`.
- `get_secret_info` returns the root `value` and `description`, plus a `has_remote_value: bool` flag that tells you whether a remote variant exists. The remote value itself is intentionally not returned by this tool.
- `compile_template_yaml` NEVER returns secret values either: every `${secret_id}` becomes `SECRET_<id>_VALUE` or `SECRET_<id>_NOT_FOUND`. To read a real value, use `get_secret_info`.

# Recommended workflow when wiring up an app from chat

1. **Discover scope** — call `list_products` to see what products already exist (each entry includes counts of templates and secrets). Pick the right `product_id`, or use `"Shared"` for cross-product configuration.

2. **Discover available secrets** — call `list_secrets` with that `product_id`. Read the `description` field on each entry to figure out which secret matches the AI's intended use. Pass `include_shared: true` to also see fallback secrets from the Shared scope.

3. **Read a specific secret** (when needed) — call `get_secret_info` to read a single secret's value, description, level, and remote-variant flag.

4. **Read a template** — call `compile_template_yaml` with `(product_id, template_id)`. The returned YAML has every `${...}` rewritten as `SECRET_<id>_VALUE` (resolvable) or `SECRET_<id>_NOT_FOUND` (missing). Compare the rendered YAML against the model/struct the app code defines, and inspect `missing_keys` to find references the template makes to non-existent secrets — these are the most common drift between "what the template asks for" and "what the secret store provides".

5. **Create or update a template** — call `upsert_template` with `(product_id, template_id, yaml)`. The call performs create-or-overwrite. To verify, follow up with `compile_template_yaml`. If a placeholder you wrote references a missing secret, the next compile will surface it.

# Conventions

- Identifiers (`product_id`, `template_id`, `secret_id`) are case-sensitive and stored verbatim. The string `"Shared"` is the only value that is matched case-insensitively (because it names the special scope).
- Empty strings are validated as errors. Pass `"Shared"` to address the shared scope; never an empty string.
- Tool errors come back as plain strings explaining what went wrong (e.g. "Template Foo/Bar not found"). Surface them to the user verbatim — they are written to be human-readable.
