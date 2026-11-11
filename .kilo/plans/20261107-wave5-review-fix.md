# Wave 5 Review Fix Plan

**Plan path:** `.kilo/plans/20261107-wave5-review-fix.md`  
**Review scope:** commits `ca8b65f` + `272b98f` on `feat/wave-5-pan-then-navigate`  
**Authority:** B-Group Step 4.3 code review of `.kilo/plans/20261107-wave5-pan-navigate-implementation-plan.md`

---

## Findings Summary

1. **Compile error in `src/app/pan.rs`** — `ArrowKeyPress` is passed by value to `arrow_pan_inputs` and then `press.direction` is used again in the error branch and inside `arrow_pan_inputs`. `NavigationDirection` is not `Copy`, so the compiler rejects the double move (`E0382`).
2. **Linux-harness test failures in `src/app/pan_tests.rs`** — tests 8, 10, and 11 assert a *successful* open after edge navigation. On the Linux container target `platform::decode_image` is the stub that always returns `ImageLoadFailure::Unloadable`, so `open_image` fails and the assertions on `opened_image_path` / `pan_offset` reset cannot pass.

All other plan requirements (math constants, routing table, paint clamp, reset semantics, file set, rule budgets) are correct.

---

## Fix 1 — Make the pan seam compile

The implementation plan's `handle_arrow_key` / `arrow_pan_inputs` structure implicitly requires both `ArrowKeyPress` and its `NavigationDirection` field to be `Copy`. The minimal, behavior-preserving fix is to add the standard derives to the two data-less types.

### Edit 1a — `src/app/folder_navigation.rs` line 24

Add `#[derive(Clone, Copy)]` above `NavigationDirection`:

```rust
/// Arrow directions of the shared navigation seam: the image axis steps the
/// current folder's sibling images; the folder axis steps the sibling folders
/// under the same parent.
#[derive(Clone, Copy)]
pub(super) enum NavigationDirection {
    Previous,
    Next,
    PreviousSiblingFolder,
    NextSiblingFolder,
}
```

Rationale: fieldless enum; existing callers already pass by value (`navigate(direction)`); adding `Copy` only removes the move restriction and has no runtime effect.

### Edit 1b — `src/app/pan.rs` line 16

Add `#[derive(Clone, Copy)]` above `ArrowKeyPress`:

```rust
/// One arrow-key press handed to the controller: the navigation direction the
/// key maps to and whether the event is a DISTINCT (non-repeat) press.
#[derive(Clone, Copy)]
pub(super) struct ArrowKeyPress {
    pub(super) direction: NavigationDirection,
    pub(super) non_repeat: bool,
}
```

Rationale: the struct is pure data, used as an immutable request object, and the plan's helper signatures pass it by value more than once.

---

## Fix 2 — Adjust tests 8, 10, 11 for the Linux decode stub

These tests must assert what the harness can verify: the navigation seam resolves the right target, `navigate` is invoked, and the decode fails with the standard message while leaving the current image in place. The "open resets pan" semantics are already covered by the passing test `every_successful_open_resets_the_pan_offset` (`src/app/pan_tests.rs` line 30).

Add the standard failure-message import to `src/app/pan_tests.rs` (after the existing `use crate::pan;`):

```rust
use crate::image_loading::UNSUPPORTED_FILE_MESSAGE;
```

### Edit 2a — test 8 `a_distinct_press_at_the_right_edge_navigates_to_the_next_image`

In `src/app/pan_tests.rs`, replace the `let next_press` block and the two success assertions (current lines 107-114) with the Linux-verifiable pattern from `src/app/tests.rs:146-161` (`failed_neighbor_open_keeps_the_current_image_and_shows_the_message`):

```rust
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Next),
        Some(folder.image_path("b.png"))
    );
    let next_press = ArrowKeyPress {
        direction: NavigationDirection::Next,
        non_repeat: true,
    };
    app.handle_arrow_key(egui::vec2(800.0, 300.0), next_press);
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_eq!(app.opened_image_path, Some(folder.image_path("a.png")));
}
```

This proves: (1) the navigation target resolves to `b.png`, and (2) the edge press routes through `navigate` → `open_image`, which fails on the Linux stub exactly like the existing neighbor-open failure test.

### Edit 2b — test 10 `a_distinct_press_at_the_bottom_edge_navigates_to_the_next_sibling_folder`

Replace the final assertion in `src/app/pan_tests.rs` (current line 148) with the vertical failure pattern from `src/app/folder_navigation_continuation_tests.rs:94-115` (`vertical_navigation_to_a_folder_whose_last_viewed_image_vanished_reports_the_standard_failure`):

```rust
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::NextSiblingFolder),
        Some(scope.child_folder_path("folder2"))
    );
    assert_eq!(
        app.sibling_folder_target_image_path(&scope.child_folder_path("folder2")),
        Some(scope.child_image_path("folder2", "b.png"))
    );
    let down_press = ArrowKeyPress {
        direction: NavigationDirection::NextSiblingFolder,
        non_repeat: true,
    };
    app.handle_arrow_key(available_size, down_press);
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_eq!(app.opened_image_path, Some(scope.child_image_path("folder1", "a.png")));
}
```

This proves: the Wave-3 vertical target resolves to `folder2/b.png`, and the bottom-edge press invokes the vertical navigation seam, which attempts the open and fails on the Linux stub.

### Edit 2c — test 11 `a_fitting_image_arrow_navigates_directly`

In `src/app/pan_tests.rs`, replace the `let next_press` block and the two success assertions (current lines 155-162) with the same Linux-verifiable navigation-attempt pattern:

```rust
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Next),
        Some(folder.image_path("b.png"))
    );
    let next_press = ArrowKeyPress {
        direction: NavigationDirection::Next,
        non_repeat: false,
    };
    app.handle_arrow_key(egui::vec2(800.0, 300.0), next_press);
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_eq!(app.opened_image_path, Some(folder.image_path("a.png")));
}
```

This preserves the behavior claim: a fitting axis routes directly to navigation even when the press is a repeat (`non_repeat: false`), and the target resolution is verified before the press.

---

## Verification

1. Run `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked` — must pass (compile error gone).
2. Run `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo test --locked` — must pass; expected test census = 150 unit tests.
3. Run the full harness: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` — must end with `ALL CHECKS PASSED`.

---

## Notes

- `src/app/folder_navigation.rs` is touched only to add `Copy`; this is the minimal correction required by the plan's own helper signatures, not a scope expansion.
- No behavior of the pan controller, navigation seam, paint clamp, or reset semantics changes.
- Floating controls, ESC-close popup, fullscreen, and folder navigation modules remain untouched, consistent with the implementation plan's Step 7.

(End of plan)
