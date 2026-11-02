# Task-3 Plan Addendum — test-module split (b2 adjudication, blocker from 4.2 preflight)

- Date: 2026-11-01
- Base plan (binding, otherwise untouched): [`20261101-wave3-task3-vertical-nav-navigation-v2.md`](20261101-wave3-task3-vertical-nav-navigation-v2.md)
- Blocker: §3.9's 9 locked tests cannot fit one file under the hard 200-line cap (real-computed floor ≈ 230).
- Adjudication (Planner): apply the implementer's recommended structural option 1 — split the test module. NO locked test, name, scenario, assertion target, or fixture behavior changes; ONLY the file placement of tests 6–9 changes.

## Delta vs v2 plan (applies ONLY to §3.8/§3.9 file placement, §3.2 ledger item 15, §6 staging list)

1. `src/app/folder_navigation_tests.rs` — contains the `TempParentScope` fixture, the shared helper `app_opened_in`, and locked tests 1–5 of §3.9 (names/semantics char-for-char). The fixture and the helper are `pub(super)` (test-only reachability, no dead-code marker needed: used across the two test modules and never compiled outside `#[cfg(test)]`).
2. NEW `src/app/folder_navigation_continuation_tests.rs` — contains locked tests 6–9 of §3.9 verbatim; imports via `use super::folder_navigation_tests::{TempParentScope, app_opened_in};` plus `use super::tests::sample_decoded_image;` where a test needs a second simulated success (test 7: two OK-accepts; test 9: two OK-accepts). Same fixture shape, same helpers; nothing duplicated.
3. `src/app.rs` §3.2 ledger item 15 gains: after `mod folder_navigation_tests;` also `#[cfg(test)] mod folder_navigation_continuation_tests;` (+ adjacent blank). Final ballot: 197–199 ≤ 200 (contingency §3.2 stands).
4. Cleanup of the stale reference: the v2 plan's test-set name `folder_navigation_tests` that appeared as "census 50 = ... folder_navigation_tests 9" reads as 5 + 4 across the two files (5 in `/folder_navigation_tests.rs`, 4 in `/folder_navigation_continuation_tests.rs`). Census stays **50**.
5. §6 staging list: the "10 paths" becomes **11 paths** — the two test files above are both staged; every other instruction (commit message, markers, invariants, checklists) remains EXACTLY as in the v2 plan.
6. Reviewer guidance (§8) additions: `folder_navigation_continuation_tests` may import `sample_decoded_image` plus the two cross-file fixture items — that is the sanctioned pattern, NOT a scope violation; `grep "put fixture"` n/a. All other greps unchanged.
