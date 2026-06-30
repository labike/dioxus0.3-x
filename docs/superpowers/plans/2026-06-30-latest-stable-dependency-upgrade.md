# Latest Stable Dependency Upgrade Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Upgrade the `0.7version` workspace to current stable dependencies and adapt business code until the workspace builds again.

**Architecture:** Keep the current shared-contract, backend-service, and frontend-web split intact while upgrading dependency declarations and repairing compile-time integration points layer by layer. Use compiler-guided fixes from lower-level crates upward so each layer stabilizes before the next one is touched.

**Tech Stack:** Rust workspace, Dioxus web, Axum, Diesel, bb8, reqwest, wasm-bindgen/web-sys, Tokio, Chrono, Time

---

### Task 1: Align dependency manifests

**Files:**
- Modify: `Cargo.toml`
- Modify: `frontend/Cargo.toml`
- Modify: `backend/server/Cargo.toml`
- Modify: `backend/query/Cargo.toml`
- Modify: `backend/crypto/Cargo.toml`
- Modify: `shared/domain/Cargo.toml`
- Modify: `shared/endpoint/Cargo.toml`

- [ ] **Step 1: Update dependency versions to current stable releases**

Set the dependency versions to the approved stable boundary from the design spec, including `time = "0.3.52"`, `reqwest = "0.13.4"`, `uuid = "1.23.4"`, `web-sys = "0.3.103"`, `wasm-bindgen = "0.2.126"`, and `bb8-diesel = "0.2.1"` from crates.io.

- [ ] **Step 2: Keep the dependency shape intentionally minimal**

Do not introduce prerelease versions or architecture-changing dependencies. Preserve existing features unless the compiler requires a feature adjustment.

- [ ] **Step 3: Run manifest-level verification**

Run: `cargo check`
Expected: dependency resolution progresses farther than the current `time-macros` failure and exposes compile-time integration errors instead.

### Task 2: Stabilize shared crates

**Files:**
- Modify: `shared/domain/src/lib.rs`
- Modify: `shared/domain/src/ids.rs`
- Modify: `shared/domain/src/user.rs`
- Modify: `shared/domain/src/post.rs`
- Modify: `shared/endpoint/src/lib.rs`
- Modify: `shared/endpoint/src/user.rs`
- Modify: `shared/endpoint/src/post.rs`
- Modify: `shared/endpoint/src/trending.rs`
- Test: `cargo check -p uchat_domain -p uchat_endpoint`

- [ ] **Step 1: Capture shared-layer failures**

Run: `cargo check -p uchat_domain -p uchat_endpoint`
Expected: focused compiler output for type, derive, feature, or serialization issues in the shared crates.

- [ ] **Step 2: Apply minimal type and derive fixes**

Adjust derives, imports, and feature-dependent types only as needed to satisfy the upgraded dependency APIs while preserving serialized behavior.

- [ ] **Step 3: Verify the shared crates build**

Run: `cargo check -p uchat_domain -p uchat_endpoint`
Expected: both crates pass compilation cleanly.

### Task 3: Upgrade backend query integration

**Files:**
- Modify: `backend/query/Cargo.toml`
- Modify: `backend/query/src/lib.rs`
- Modify: `backend/query/src/util.rs`
- Modify: `backend/query/src/error.rs`
- Modify: `backend/query/src/user.rs`
- Modify: `backend/query/src/post.rs`
- Modify: `backend/query/src/session.rs`
- Modify: `backend/query/src/test_db.rs`
- Modify: `backend/query/src/schema.rs`
- Test: `cargo check -p uchat_query`

- [ ] **Step 1: Reproduce query-layer failures**

Run: `cargo check -p uchat_query`
Expected: compile errors around Diesel, migrations, connection pooling, or trait bounds after the dependency upgrade.

- [ ] **Step 2: Fix the `bb8-diesel` and Diesel integration**

Update imports, pool aliases, connection manager usage, and migration harness calls to match the stable crates.io `bb8-diesel` and current Diesel APIs.

- [ ] **Step 3: Repair query methods and typed models**

Adjust query functions, insert/update structs, and trait imports only where the compiler indicates incompatibilities.

- [ ] **Step 4: Verify the query crate**

Run: `cargo check -p uchat_query`
Expected: the crate builds successfully, or only environment-gated tests remain.

### Task 4: Repair backend server integration

**Files:**
- Modify: `backend/server/src/lib.rs`
- Modify: `backend/server/src/router.rs`
- Modify: `backend/server/src/extractor.rs`
- Modify: `backend/server/src/error.rs`
- Modify: `backend/server/src/handler.rs`
- Modify: `backend/server/src/handler/user.rs`
- Modify: `backend/server/src/handler/post.rs`
- Modify: `backend/server/src/bin/api.rs`
- Test: `cargo check -p uchat_server`

- [ ] **Step 1: Reproduce server-layer failures**

Run: `cargo check -p uchat_server`
Expected: any remaining Axum, extractor, handler, or connection abstraction errors surface clearly.

- [ ] **Step 2: Adapt server code to the stabilized query layer**

Fix extractor signatures, pool access paths, handler imports, and router glue to match the updated query abstractions and current dependency APIs.

- [ ] **Step 3: Verify the server crate**

Run: `cargo check -p uchat_server`
Expected: the server crate compiles successfully.

### Task 5: Repair frontend business code

**Files:**
- Modify: `frontend/Cargo.toml`
- Modify: `frontend/src/main.rs`
- Modify: `frontend/src/app.rs`
- Modify: `frontend/src/util.rs`
- Modify: `frontend/src/util/api_client.rs`
- Modify: `frontend/src/page.rs`
- Modify: `frontend/src/page/home.rs`
- Modify: `frontend/src/page/home/bookmarked.rs`
- Modify: `frontend/src/page/home/liked.rs`
- Modify: `frontend/src/page/trending.rs`
- Modify: `frontend/src/page/login.rs`
- Modify: `frontend/src/page/register.rs`
- Modify: `frontend/src/page/view_profile.rs`
- Modify: `frontend/src/page/edit_profile.rs`
- Modify: `frontend/src/page/new_post/chat.rs`
- Modify: `frontend/src/page/new_post/image.rs`
- Modify: `frontend/src/page/new_post/poll.rs`
- Modify: `frontend/src/elements/post.rs`
- Modify: `frontend/src/elements/post/action_bar.rs`
- Modify: `frontend/src/elements/post/content.rs`
- Modify: `frontend/src/elements/toaster.rs`
- Test: `cargo check -p frontend`

- [ ] **Step 1: Reproduce frontend failures**

Run: `cargo check -p frontend`
Expected: compiler errors point to `reqwest`, wasm/web bindings, hooks usage, or page/component API mismatches.

- [ ] **Step 2: Fix request and platform integration**

Update the HTTP client, timeout handling, URL building, and wasm/web interop code to match the upgraded stable dependency APIs.

- [ ] **Step 3: Fix component and page business code**

Repair any page/component signatures, async hooks, and route glue that fail after the dependency refresh while preserving existing product behavior.

- [ ] **Step 4: Verify the frontend crate**

Run: `cargo check -p frontend`
Expected: the frontend crate compiles successfully.

### Task 6: Workspace verification and review

**Files:**
- Modify: `Cargo.lock`
- Test: `cargo check`
- Test: `cargo test -p uchat_query` (if environment permits)

- [ ] **Step 1: Run full workspace verification**

Run: `cargo check`
Expected: the entire workspace compiles successfully.

- [ ] **Step 2: Run additional targeted tests where possible**

Run: `cargo test -p uchat_query`
Expected: tests pass if local Postgres and migration prerequisites are available; otherwise capture the exact environment blocker.

- [ ] **Step 3: Review the resulting change set**

Confirm that the Git dependency was removed, the stabilized version boundary was respected, and no prerelease crates were introduced.

- [ ] **Step 4: Summarize residual risks**

Document any remaining runtime-only or environment-gated validation that still needs a live database or browser run.
