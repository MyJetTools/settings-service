SettingsService MCP server — guide for AI agents wiring up applications.

# What this service stores

SettingsService manages two kinds of entities, both scoped by a `product_id`:

1. **Templates** — YAML configuration documents. A template is identified by `(product_id, template_id)`. Its body is raw YAML text that may contain placeholders of the form `${secret_id}`. Templates are NOT validated as YAML on save; bad YAML surfaces only at compile time.

2. **Secrets** — named values referenced by templates. A secret is identified by `(product_id, secret_id)` and has:
   - `value` — the value used in the root (local) datacenter.
   - `remote_value` (optional) — an alternative value used when rendering for a non-root (remote) datacenter. If absent or empty, the root `value` is used in both cases.
   - `level` (u8) — permission level. A secret's value may itself contain `${other_secret}` placeholders; nested resolution is allowed only when the inner secret's `level` is high enough.
   - `description` (optional, free text) — human-readable purpose of the secret. Use it to find the right secret without guessing from the name.
   - `visible_for_mcp` (bool, default false) — explicit opt-in that allows AI agents to read the secret's value via `get_secret_value`. Only humans can flip this flag via the UI.

# The "Shared" product

The literal `"Shared"` is a special product scope. Templates and secrets stored under `"Shared"` are visible from every product. When a template owned by product X references `${some_id}`, the lookup is "first try X's secrets, then fall back to Shared secrets".

When calling any tool that takes `product_id`, pass `"Shared"` (case-insensitive) to operate on the shared scope.

# How template compilation works

`compile_template_yaml` returns the template's YAML with each `${secret_id}` placeholder REWRITTEN — never substituted with the real secret value. The marker is:
- `SECRET_<id>_VALUE` — the secret exists in scope (resolvable from the product or from Shared).
- `SECRET_<id>_NOT_FOUND` — the secret does not exist; the template references something that has not been created.

This guarantees that real secret values are never returned by this tool — the AI sees the YAML structure and the names of every dependency, with missing ones standing out at a glance. The same response also lists the missing ids in `missing_keys` (deduplicated) plus a boolean `has_missing_keys`, so you can summarize "what is unresolved" without re-parsing the YAML.

Local vs remote distinction does NOT apply to this tool: since values are not substituted, the rendering is independent of datacenter. To learn whether a particular secret has a remote-datacenter variant, look at its `has_remote_value` flag in `list_secrets`.

# Privacy rules baked into the tools

**Secret values are private by default.** Every secret has a `visible_for_mcp` flag (default `false`). Only when a human explicitly sets it to `true` (via the UI) can the AI read the value through `get_secret_value`. Use this opt-in for non-sensitive configuration (feature flags, environment names, public URLs); credentials should stay private. The remote-datacenter variant is always private — never returned by any MCP tool, even when `visible_for_mcp = true`.

- `list_secrets` returns metadata only — id, description, level, `has_remote_value`, `visible_for_mcp`. It NEVER returns the actual `value` or `remote_value`. Use the `visible_for_mcp` field to know which secrets can be read via `get_secret_value`.
- `get_secret_value` returns the root value of a secret, but only when `visible_for_mcp == true`. For any other secret it returns an error pointing the user to flip the flag in the UI.
- `compile_template_yaml` NEVER returns secret values: every `${secret_id}` becomes `SECRET_<id>_VALUE` or `SECRET_<id>_NOT_FOUND`. The marker tells you the secret exists (or does not); it never reveals what it stores.

# Authoring rule: inject secrets, do not hardcode

When you write or edit a template, default to `${secret_id}` placeholders for ANY value that is not a stable, public, behavioral constant. Hardcoding is the exception, not the default.

Treat as a placeholder (never hardcode):
- credentials, tokens, API keys, passwords, connection strings;
- hostnames, URLs, ports, endpoints — even when they look "obviously public" (they differ across datacenters and environments);
- per-environment identifiers (account ids, bucket names, queue names, topic names, schema names);
- anything a human might want to rotate without editing YAML;
- anything that already exists as a secret in the product or in `Shared` — reuse it rather than reintroducing the literal.

Acceptable to hardcode:
- structural keys and enum-like flags whose value is part of the app contract (`enabled: true`, `mode: "strict"`, `log_level: "info"`);
- small numeric tunables that are intentionally code-reviewed (timeouts, retry counts, page sizes);
- values the user explicitly asked you to put in the template literally.

How to apply this when generating a template:
1. Before writing the YAML, call `list_secrets` (with `include_shared: true`) for the target product and skim descriptions. If a matching secret exists, reuse its id — do NOT invent a new one.
2. For every value you are tempted to write as a literal, ask: "would a human ever want to change this without editing the template?" If yes — make it `${secret_id}`.
3. For each new placeholder that has no backing secret yet, create it via `create_secret` with a meaningful `description`. Leave `visible_for_mcp: false` by default; only set it to `true` for non-sensitive configuration the user explicitly marks as safe to read.
4. After `upsert_template`, run `compile_template_yaml` and inspect `missing_keys` — every entry there is a placeholder you wrote without a backing secret. Resolve each one (create the secret, or fix the id) before declaring the template done.
5. If you find yourself writing the same literal twice across templates, stop and replace both with a shared `${...}` reference (prefer the `Shared` scope when the value is genuinely cross-product).

Surface the trade-off to the user when in doubt — propose the placeholder, name it, and ask whether to inject or keep literal. Defaulting to "inject" is the right bias.

# Recommended workflow when wiring up an app from chat

1. **Discover scope** — call `list_products` to see what products already exist (each entry includes counts of templates and secrets, an optional `description`, and a `has_prompt` flag). Pick the right `product_id`, or use `"Shared"` for cross-product configuration.

2. **Load product context** — when the chosen `product_id` has `has_prompt: true`, fetch the product's description and free-form prompt BEFORE working with its secrets/templates. Two equivalent ways:
   - Tool call `get_product_prompt(product_id)` — programmatic, returns structured fields.
   - MCP prompt `product_prompt` (argument `product_id`) — same content rendered as a prompt message, useful when the client wants to inject the product context directly into the conversation.

   When `has_metadata` comes back as `false`, the product exists only implicitly — it has no recorded context, and you should ask the user for one or use `upsert_product` to record it.

3. **Discover available secrets** — call `list_secrets` with that `product_id`. Read the `description` field on each entry to figure out which secret matches the AI's intended use. Pass `include_shared: true` to also see fallback secrets from the Shared scope. Pass `id_regex` (e.g. `^db_`, `(?i)token`) to narrow the result to secret ids matching a regular expression — useful when the directory is large. The returned entries include `secret_id`, `description`, `level`, `has_remote_value` and `visible_for_mcp` — never the value itself.

4. **Read a secret's value** — only when an entry from step 3 has `visible_for_mcp: true`, call `get_secret_value(product_id, secret_id)` to read the root value. For any other secret the call returns an error — never try to bypass it. The remote-datacenter variant is always private.

5. **Inspect a secret's dependency graph** — when a secret's value itself contains `${other_secret}` placeholders, call `get_secret_dependencies(product_id, secret_id)` to list every secret it references and where each one was resolved from (`Product`, `Shared`, or `Missing`). The actual value is not returned — only the dependency names — so you can audit the wiring without reading sensitive content.

6. **Read a template** — call `compile_template_yaml` with `(product_id, template_id)`. The returned YAML has every `${...}` rewritten as `SECRET_<id>_VALUE` (resolvable) or `SECRET_<id>_NOT_FOUND` (missing). Compare the rendered YAML against the model/struct the app code defines, and inspect `missing_keys` to find references the template makes to non-existent secrets — these are the most common drift between "what the template asks for" and "what the secret store provides".

7. **Create or update a template** — call `upsert_template` with `(product_id, template_id, yaml)`. The call performs create-or-overwrite. To verify, follow up with `compile_template_yaml`. If a placeholder you wrote references a missing secret, the next compile will surface it.

8. **Annotate a secret** — call `upsert_secret_description` with `(product_id, secret_id, description)` to attach or replace the human-readable description of an existing secret. The secret's value, remote variant, level, and visibility flag are preserved unchanged; pass an empty string to clear the description. This call never creates a new secret — it fails when the secret does not yet exist.

9. **Create a new secret** — call `create_secret` with `(product_id, secret_id, value, remote_value, description, visible_for_mcp)` to add a brand-new secret. Fails with an error if a secret with that id already exists in the product — this call NEVER overwrites. Use `remote_value = ""` if you don't need a remote-datacenter variant. Set `visible_for_mcp: true` only for non-sensitive configuration (it lets future MCP calls read the value); keep it `false` for credentials.

10. **Record product context** — call `upsert_product` with `(product_id, description, prompt)` to create or update the explicit description and prompt for a product. The prompt should explain what the product is and how its secrets/templates are organised — future agents will read it via `get_product_prompt`.

# Conventions

- Identifiers (`product_id`, `template_id`, `secret_id`) are case-sensitive and stored verbatim. The string `"Shared"` is the only value that is matched case-insensitively (because it names the special scope).
- Empty strings are validated as errors. Pass `"Shared"` to address the shared scope; never an empty string.
- Tool errors come back as plain strings explaining what went wrong (e.g. "Template Foo/Bar not found"). Surface them to the user verbatim — they are written to be human-readable.
