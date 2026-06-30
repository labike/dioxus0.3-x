# Latest Stable Dependency Upgrade Design

**Date:** 2026-06-30

**Scope:** Upgrade the `0.7version` branch to current stable Rust crate releases where available, and adapt business code across shared crates, backend, and frontend so the workspace builds again.

## Goal

Bring the workspace onto the newest stable dependency line that is compatible with the current architecture, avoiding prerelease crates, and restore a working build through code changes rather than manifest-only edits.

## Constraints

- Use `0.7version` as the implementation baseline.
- Prefer latest stable releases from crates.io, not alpha, beta, rc, or Git-only sources when a stable crates.io release exists.
- Keep the current product architecture: `shared/*` contracts, `backend/*` services, and `frontend` Dioxus web app.
- Completion target is `cargo check` success for the workspace, plus any additional tests that can run in the local environment.

## Dependency Boundary

The latest stable versions confirmed for the main stack on 2026-06-30 are:

- `dioxus`, `dioxus-router`, `dioxus-web`: `0.7.9`
- `axum`: `0.8.9`
- `axum-extra`: `0.12.6`
- `diesel`: `2.3.10`
- `diesel_migrations`: `2.3.2`
- `reqwest`: `0.13.4`
- `tokio`: `1.52.3`
- `chrono`: `0.4.45`
- `time`: `0.3.52`
- `uuid`: `1.23.4`
- `web-sys`: `0.3.103`
- `wasm-bindgen`: `0.2.126`
- `bb8-diesel`: `0.2.1`

The implementation will not move to prerelease lines such as `dioxus 0.8.0-alpha`, `rsa 0.10.0-rc`, or `argon2 0.6.0-rc`.

## Current Root Causes

1. The workspace no longer resolves cleanly because `frontend` pins `time = 0.3.51`, which requires a `time-macros` release that is not currently available from crates.io in this environment.
2. Several dependency versions drift across crates, which increases lockfile instability and rebuild churn.
3. `backend/query` still depends on a Git source for `bb8-diesel`, even though a stable crates.io release exists.
4. The codebase spans multiple tightly-coupled layers, so compile fixes will need to propagate through shared types, backend query/server code, and frontend request/UI code.

## Architecture Impact

### Shared crates

`shared/domain` and `shared/endpoint` remain the API boundary between frontend and backend. Version upgrades here should stay minimal and preserve serialized wire contracts unless a compile fix requires a signature change.

### Backend

`backend/query` is the most dependency-sensitive layer because it touches Diesel, migrations, bb8 pooling, and query helper types. `backend/server` depends on those query-layer abstractions through extractors and handlers.

### Frontend

The frontend remains on the Dioxus `0.7.9` stable line. Expected business-code changes are more likely around request/client behavior, async glue, and any web/wasm ecosystem updates than around a Dioxus major migration.

## Upgrade Strategy

1. Normalize dependency declarations to current stable releases.
2. Replace the Git `bb8-diesel` dependency with the stable crates.io release.
3. Regenerate dependency resolution and capture the first complete compiler error set.
4. Fix compile issues from the bottom up:
   - `shared/*`
   - `backend/query`
   - `backend/server`
   - `frontend`
5. Preserve user-facing behavior while adjusting business code to the updated dependency APIs.

## Verification Strategy

- Run `cargo check` for the workspace after each significant phase.
- Prefer targeted crate checks first when errors are localized.
- Run available tests that do not require unavailable external services.
- If database-dependent tests are blocked by environment setup, report that explicitly after compile verification.

## Risks

- Diesel and pool integration may require multiple rounds of type or trait fixes.
- Frontend `reqwest`/wasm interaction may expose API mismatches once the lockfile updates.
- Existing branch code already appears mid-migration in places, so some errors may be unrelated to version bumps and still need repair.

## Success Criteria

- Workspace manifests use current stable crate versions within the approved boundary.
- The project no longer depends on the Git version of `bb8-diesel`.
- Business code is updated as needed to compile against the upgraded dependencies.
- `cargo check` succeeds for the workspace, or any remaining blockers are limited to external environment constraints and are clearly documented.
