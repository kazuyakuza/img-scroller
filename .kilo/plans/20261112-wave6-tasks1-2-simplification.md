# Wave 6 — Tasks 1+2 Simplification Plan (4.3 code-simplifier)

- Date: 2026-11-12 · Branch: `feat/wave-6-open-with` · Reviewed commit: `bfa20e1`
- Verdict: **SIMPLIFICATION REQUIRED** — one tiny, local, test-only change. No production-code change.
- Binding constraints respected: enumeration/launcher mechanisms, the seam's public shape, stub behaviors, and pinned test semantics/`&Vec::new()` spelling are untouched; no `#[allow(...)]` removal (per §9 handoff notes).

## Change 1 (ONLY change): dedup the display-name extraction in `src/open_with_listing/tests.rs`

Problem: tests `handler_names_map_to_programs_in_system_order` (lines 24–28) and
`non_ascii_display_names_are_preserved_verbatim` (lines 66–70) contain an identical
4-line `map(|program| program.display_name.as_str()).collect()` block that duplicates
the style of the existing `names_and_indices` helper (lines 3–8). The plan (Step 3.2)
pins only the six test names, fixtures, and assertion semantics — bodies are open.

Fix, applied as three exact edits to `src/open_with_listing/tests.rs`:

1. Add this helper immediately after `names_and_indices` (before the first `#[test]`):

```rust
fn display_names(programs: &[RegisteredProgram]) -> Vec<&str> {
    programs
        .iter()
        .map(|program| program.display_name.as_str())
        .collect()
}
```

2. Replace the whole body of `handler_names_map_to_programs_in_system_order` with:

```rust
    let handler_names = vec![
        "Zed Viewer".to_string(),
        "Alpha Paint".to_string(),
        "Photos".to_string(),
    ];
    let programs = programs_from_handler_names(&handler_names);
    assert_eq!(
        display_names(&programs),
        ["Zed Viewer", "Alpha Paint", "Photos"]
    );
```

3. Replace the whole body of `non_ascii_display_names_are_preserved_verbatim` with:

```rust
    let handler_names = vec!["写真ビューア".to_string(), "Photos".to_string()];
    let programs = programs_from_handler_names(&handler_names);
    assert_eq!(display_names(&programs), ["写真ビューア", "Photos"]);
```

No import changes (`RegisteredProgram` is already imported on line 1). No other file is
touched. Test names, fixtures, and asserted values stay byte-identical in meaning.

## Explicitly REJECTED (do not act on these)

- `extension_query_string` capacity `+ 2`: self-evident (dot + NUL), plan-pinned shape — keep.
- `drained_handlers` loop → `std::iter::from_fn(...).collect()`: plan §3.5 compliance notes
  explicitly justify the flat loop; reversing an architector-justified shape is out of scope.
- Triple `ok_or(OpenWithLaunchFailure::LaunchFailed)` in the launcher: idiomatic; a helper
  would add indirection, not remove it.
- Inner `unsafe { ... }` blocks in `drained_handlers`/`next_handler`/`handler_ui_name`/
  `file_data_object`: REQUIRED under edition 2024 + clippy `-D warnings` (`unsafe_op_in_unsafe_fn`);
  the landed form is already minimal. Removing them or marking bodies `#[allow(unsafe_op_in_unsafe_fn)]`
  would be a regression.
- `encode_wide().chain([0])` and `CoTaskMemFree(...cast::<core::ffi::c_void>())` appear once
  each in `windows_open_with.rs`; the second site of each is pre-existing `windows.rs` code —
  a cross-module extraction is out of the Tasks-1+2 commit scope (plan §6/§7).
- `&Vec::new()` in `empty_input_yields_no_programs`: pinned spelling per plan Step 3.2 item 1 — keep.
- All `#[allow(dead_code)]` / `#[allow(unused_imports)]`: intentional per plan §9 — keep.

## Verification the implementer must run (after applying)

Per `docs/build.md` on the alpine-vm (vm_status first; exit code authoritative): the dev-checks
harness must report `ALL CHECKS PASSED` (fmt/check/test/clippy). Expected: 170 tests / 27 test
files unchanged (6 tests in `open_with_listing/tests.rs`, same names, same semantics).
