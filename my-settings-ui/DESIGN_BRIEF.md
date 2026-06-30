# Design Brief — Settings Service Admin UI

Instruction for a Claude "design" session. Your job is to produce a coherent
**visual & UX design** for the Settings Service web admin (the `my-settings-ui`
Dioxus app), then express it as concrete, implementable artifacts (design tokens,
component styles, and per-screen layout specs) — **not** a from-scratch rewrite.

Read this whole brief before proposing anything. When something is ambiguous,
ask one round of clarifying questions; otherwise follow the defaults stated here.

---

## 1. What the app is

A small internal **admin tool** for ops/engineers to manage application
configuration. It stores two kinds of entities, both scoped by a `product_id`,
across one or more **environments** (`env`):

- **Secrets** — named values (`product_id` + `secret_id`). Each has a root
  `value`, an optional `remote_value` (used for remote datacenters), a numeric
  `level` (permission tier), an optional `description`, and a `visible_for_mcp`
  flag (lets AI agents read the value).
- **Templates** — YAML config documents (`product_id` + `template_id`) whose
  body contains `${secret_id}` placeholders that get substituted at request time.
- **Products** — the namespace that owns secrets/templates. A product may carry
  explicit metadata (description + AI prompt) or exist only **implicitly**.
- **`Shared`** — a special product scope visible from every product. Secrets and
  templates here act as cross-product fallbacks.

The audience is technical, internal, and data-focused. This is a **dense data
admin**, not a marketing site or consumer app. Optimize for scanning, comparing,
and editing many rows quickly — not for whitespace-heavy elegance.

---

## 2. Hard constraints (do not violate)

The design must be implementable in the existing stack with reasonable effort:

- **Dioxus 0.7, web/WASM, SPA.** Rendered as RSX (HTML-like) in Rust. No React,
  Tailwind build step, SCSS pipeline, or JS framework. Styling ships as plain CSS.
- **Bootstrap 5 is already loaded** (`bootstrap.css` + `bootstrap.js`). Reuse its
  classes (`table table-striped`, `btn btn-sm btn-*`, `badge text-bg-*`,
  `input-group`, `modal`, `toast`) rather than inventing parallel systems.
- **Custom CSS lives in two files**: `public/assets/app.css` (app-specific, your
  main canvas) and `public/assets/styled.css`. Express the design as **CSS custom
  properties (design tokens)** at the top of `app.css` and class rules below.
- **Component library available**: `dioxus-admin-ui-kit` provides typed inputs
  (`InputValueComponent`), `input_bool`, enum selectors (`select_enum_value[_opt]`),
  and `RenderTable<T>`. Design with these primitives in mind so forms/tables map
  onto them. Don't design controls that can't be built from RSX + Bootstrap + this kit.
- **No new heavy dependencies, no icon webfonts.** Icons today are inline SVG
  components (`src/icons/`). Keep that approach.
- Must work as a **full-viewport two-pane layout**: fixed left nav + scrollable
  right content (see current `#layout` / `#left-panel` / `#right-panel`).

If a design idea requires breaking one of these, call it out explicitly and
propose a within-constraints alternative.

---

## 3. Current state (your starting point)

Layout: a ~200px gray left panel (`#left-panel`, `#efefef`) with the title
"Settings", an environment/user badge pinned to the bottom, and a vertical menu:
**Secrets · Templates · Products**. The right panel renders the active page as a
full-width Bootstrap striped table with a sticky-ish toolbar row in `<thead>`.

Current tokens already in `app.css`:

- `--accent-color: #40bb82` (green) — the brand accent.
- `--label-color: gray`, `--text-color: black`, `--border-color: lightgray`.
- `--shadow: 0 0 5px lightgray`, panel width `200px`.

Honest assessment of what's weak today (fix these):

- Color is used inconsistently — raw `red`/`green`/`blue`, ad-hoc inline styles,
  Bootstrap contextual classes, and the green accent all coexist with no system.
- Tables are visually flat; header toolbars mix sort controls, search, product
  selector, and add/import/export buttons in cramped `<th>` cells.
- Status signaling (usage badges, MCP, Shared, level, missing-placeholder warning,
  "Last edited", implicit/explicit product) has no consistent visual grammar.
- Dialogs are functional but plain; empty/loading/error states are bare text.

Treat the existing structure as the skeleton to **re-skin and tighten**, not to
discard. Routes, page structure, and dialog inventory stay the same.

---

## 4. Screens & components to cover

Design each of these. For tables, design the **row, header/toolbar, badges, and
action buttons** — the columns listed are real.

### 4.1 App shell
- Left nav: brand, environment selector (currently a bottom badge — promote it to
  a real selector), 3 nav items with clear active state. Consider a footer slot
  for the signed-in user.
- Right pane: page toolbar + data table + dialogs + toast region.

### 4.2 Secrets page (`/secrets`)
Columns: **Used→Templates** (clickable count badge), **Used→Secrets** (clickable
count badge), **Product scope** (badge: product id, or amber `Shared`), **Name**
(+ optional `MCP` badge, + `Last edited` badge), **Description**, **Level**,
**Created**, **Updated**, **Actions** (view, clone, edit, delete).
Toolbar: product-scope selector, name search, add button.
Key states: a secret used by nothing (today shown red) vs. used (green) — design
this "orphan vs. referenced" signal cleanly.

### 4.3 Templates page (`/templates`)
Columns: **warning** (missing-placeholder alert), **select checkbox** (for
export), **Product**, `/`, **Template id** (+ `Last edited`), **Created**,
**Updated**, **Last request** (timestamp + age), **Actions** (view populated
YAML, copy-from, edit, delete).
Toolbar: product selector, name search, add, import, export (export appears only
when rows are selected).

### 4.4 Products page (`/products`)
Columns: **Product id**, **Type** (badge: `explicit` vs `implicit`),
**Description**, **Templates** count, **Secrets** count, **Actions** (view
prompt, edit, delete — view/delete only when metadata exists).

### 4.5 Dialogs (modal, Bootstrap-based)
Edit Secret, Edit Template (with secret picker / placeholder helper), Edit
Product, Confirmation, Show populated YAML (monospace, near-full-screen), Show
secret value, Secret-usage-by-template, Secret-usage-by-secret, Show product
prompt, Snapshot import, Snapshot export. Design a **consistent dialog frame**
(header, body, footer with primary/secondary actions) plus the full-screen
monospace variant for YAML.

### 4.6 Cross-cutting states
Loading (there's a spinner `.loader`), empty list, error, and the danger toast
(`#liveToast`). Design all four — they're currently plain text.

---

## 5. Design goals

1. **Legible density.** More rows visible without feeling cramped. Tighten
   vertical rhythm, align numbers right, give tables a calm zebra + hover.
2. **A real status grammar.** One consistent system of badges/pills/colors for:
   usage (referenced / orphan), scope (`Shared` vs product), MCP-visible, level,
   product type (explicit/implicit), missing-placeholder warning, last-edited.
   Each meaning gets one color and one shape — used identically everywhere.
3. **Calm, professional palette** built around the existing green accent
   `#40bb82`. Define semantic tokens (success/warn/danger/info/neutral) instead
   of raw color names. Support a future dark mode by going token-first (don't
   hardcode `black`/`white`).
4. **Obvious primary action per screen** (Add), with destructive actions visually
   subordinate and guarded by confirmation.
5. **Coherent forms.** Inputs map onto `dioxus-admin-ui-kit` controls; clear
   label/value/validation-error pattern; disabled Save until valid.

---

## 6. What to deliver

Produce these, in order:

1. **Design rationale** — 1 short section: the visual direction and why it fits an
   internal data admin. No fluff.
2. **Design tokens** — a concrete block of CSS custom properties for `app.css`:
   color (semantic + accent ramp), typography scale, spacing scale, radius,
   border, shadow, table density, z-index. Keep names self-descriptive; replace
   the ad-hoc tokens currently there.
3. **Component specs** — for buttons, badges/pills, tables (header/toolbar/row/
   zebra/hover), inputs/forms, dialogs (standard + full-screen YAML), toast,
   loading/empty/error. For each: the Bootstrap classes to reuse + the custom CSS
   to add. Show the actual CSS, not prose.
4. **Per-screen layout specs** — for the shell + the 3 pages, an ASCII wireframe
   and notes mapping each element to a token/component above. Reference the real
   columns from §4.
5. **Status-grammar table** — every status meaning → its pill style/color/icon.
6. **Migration notes** — which current rules in `app.css` to keep, change, or
   remove; any `styled.css` overrides to reconcile. Flag anything that needs a
   matching RSX class change in `src/`.

Output format: Markdown with fenced `css` blocks ready to paste into `app.css`,
and ASCII wireframes for layout. Do **not** write Rust/RSX unless a token change
forces a class rename — in that case, list the exact `src/` edits needed.

---

## 7. Guardrails

- Don't introduce a new CSS framework, build step, or icon font.
- Don't redesign the information architecture (routes, entities, dialog set stay).
- Don't propose interactions Dioxus 0.7 + Bootstrap JS can't do cheaply.
- Keep contrast/accessibility sane (WCAG AA for text); color is never the *only*
  signal — pair it with text/icon (e.g. the `Shared` badge says "Shared").
- Stay token-first so a dark theme is a token swap, not a rewrite.

## 8. Acceptance criteria

- Every screen and dialog in §4 has a spec.
- Every status meaning in §5.2 has exactly one pill style in the §6.5 table.
- All tokens are real CSS custom properties; all component CSS is paste-ready.
- Nothing in the spec violates §2. Any tension is called out with an alternative.
