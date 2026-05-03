pub const MCP_INSTRUCTIONS: &str = r#"
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

`compile_template_yaml` produces TWO renderings of the YAML:
- `yaml` — the local (root datacenter) rendering. Every `${secret_id}` is replaced with the secret's `value`.
- `remote_yaml` — the remote-datacenter rendering. Each secret with a non-empty `remote_value` substitutes its remote variant; secrets without a remote variant fall back to their root `value`. `remote_yaml` is omitted from the response when the result is identical to `yaml` (i.e. no secret in scope has a remote variant), so an empty `remote_yaml` field MEANS "remote == local".

When a secret is missing, the placeholder is replaced inline with the comment marker `/*Secret <id> is not found*/`. The same response also returns the missing ids in `missing_keys` and a boolean `has_missing_keys`. Use `missing_keys` to summarize what is unresolved without re-parsing the YAML.

# Privacy rules baked into the tools

- `list_secrets` returns metadata only — id, description, level, `has_remote_value`. It NEVER returns the actual `value` or `remote_value`. To inspect a value, follow up with `get_secret_info`.
- `get_secret_info` returns the root `value` and `description`, plus a `has_remote_value: bool` flag that tells you whether a remote variant exists. The remote value itself is intentionally not returned by this tool. To see it rendered in context, use `compile_template_yaml`.

# Recommended workflow when wiring up an app from chat

1. **Discover scope** — call `list_products` to see what products already exist (each entry includes counts of templates and secrets). Pick the right `product_id`, or use `"Shared"` for cross-product configuration.

2. **Discover available secrets** — call `list_secrets` with that `product_id`. Read the `description` field on each entry to figure out which secret matches the AI's intended use. Pass `include_shared: true` to also see fallback secrets from the Shared scope.

3. **Read a specific secret** (when needed) — call `get_secret_info` to read a single secret's value, description, level, and remote-variant flag.

4. **Read a template** — call `compile_template_yaml` with `(product_id, template_id)`. Compare the rendered YAML against the model/struct the app code defines. Inspect `missing_keys` to find references the template makes to non-existent secrets — these are the most common drift between "what the template asks for" and "what the secret store provides".

5. **Create or update a template** — call `upsert_template` with `(product_id, template_id, yaml)`. The call performs create-or-overwrite. To verify, follow up with `compile_template_yaml`. If a placeholder you wrote references a missing secret, the next compile will surface it.

# Conventions

- Identifiers (`product_id`, `template_id`, `secret_id`) are case-sensitive and stored verbatim. The string `"Shared"` is the only value that is matched case-insensitively (because it names the special scope).
- Empty strings are validated as errors. Pass `"Shared"` to address the shared scope; never an empty string.
- Tool errors come back as plain strings explaining what went wrong (e.g. "Template Foo/Bar not found"). Surface them to the user verbatim — they are written to be human-readable.
"#;
