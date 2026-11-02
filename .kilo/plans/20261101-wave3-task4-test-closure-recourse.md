# Task-4 Recourse Plan — E0373 closure lifetime in the sibling listing test seam

- Date: 2026-11-01
- Trigger: dev-checks round 2 at Task 4 (`FAIL test exit=101`): `error[E0373]` at `src/sibling_folder_listing/tests.rs:112` and `:148` — the closures passed to `from_opened_image_with_scope_entries` capture `scope` by reference, but `type ScopeEntriesReader = dyn Fn(&Path) -> Option<Vec<ScopeEntry>>;` (bare `dyn`, no parameter) implies a `'static` trait-object bound on `&ScopeEntriesReader`, so borrowed captures cannot compile. Pre-existing at Task-1 land (`bc7261a`); first exercised now (tests run only at Task 4). `cargo check` never caught it: test cfg does not compile under plain `check`.
- Sanctioned resolution (Planner adjudication; minimal, behavior-preserving, NO test edits): parameterize the trait-object lifetime. The closures then borrow `scope` for the duration of the single private-constructor call — their actual lifetime — instead of being forced to `'static`.
- Rule anchor: `max-lines-per-file` etc. unaffected; `sibling_folder_listing/tests.rs` (at the 200 cap) stays byte-identical; plan-followers' "byte-identical" checks keep holding.

## Exact edits (the ONLY sanctioned change; 2 lines, one file)

File `src/sibling_folder_listing.rs` ONLY:

1. Line 11:
   - From: `type ScopeEntriesReader = dyn Fn(&Path) -> Option<Vec<ScopeEntry>>;`
   - To:   `type ScopeEntriesReader<'r> = dyn Fn(&Path) -> Option<Vec<ScopeEntry>> + 'r;`
2. The private constructor's parameter (inside `from_opened_image_with_scope_entries`):
   - From: `read_scope_entries: &ScopeEntriesReader,`
   - To:   `read_scope_entries: &ScopeEntriesReader<'_>,`

Nothing else changes: `ScopeEntry`, the `pub(crate)` surface, `from_opened_image`, the reader default `read_scope_entries`, all helper bodies, and BOTH test files (incl. the 200/200-capped one) stay byte-identical.

## Compiler evidence this resolves both sites

- `tests.rs:112` / `:148` closures capture `scope: &TempScope` (borrow used inside the closure body only); after the edit the required capture bound is the anonymous short lifetime `'r` imposed at each call (`&read_entries` used within the same expression scope) — E0373 gone.
- The production path is unaffected: `read_scope_entries` (the plain fn) is `Option<Vec<ScopeEntry>>`-valued and satisfies `+ 'r` for any `'r` (no captures).

## Process discipline

- Implementer: apply the 2 edits; `git diff` must show ONLY that file's 2 lines; commit EXACT message `fix: bound the scope-entries reader to the caller's lifetime`; THEN resume Task-4 4.2 verification from the plan's dev-checks step (the lock + fmt commits from the halted run stay).
- Reviewers/4.5b: record this as Planner-sanctioned recourse (source: this plan); all other Task-4 assertions unchanged. Max this fix: 1 round; a second evaluator-lifetime problem must escalate to the user.
