# Amendment Plan — Task 4 (Wave 1) implementation: two §3.3 draft compile defects + fmt-stability drift resolution

- Date: 2026-10-31. Supersedes nothing else: this is a MINI AMENDMENT to the base plan
  [`.kilo/plans/20261031-task4-wave1-display-image-implementation.md`](20261031-task4-wave1-display-image-implementation.md)
  (Task 4, TODO `.agent/todos/20261029/20261029-todo-3.md` "Task 4: Display the image").
- Amended plan sections: §3.3 (`src/app_display.rs` — TWO defects §1/§2 below), §1.2 (freeze language), §5 (commit file list), §8 (out-of-scope wording). Everything else in the base plan stays fully in force.
- Auto-approved scope note: the base plan stands ("Approve Global and Tasks Plans"); this amendment inherits auto-approval — save + return path, no user approval step.
- Implementer state when this amendment was ordered (verified evidence): S2 drafts landed verbatim; S3 `cargo fmt` exit 0; S4 dev-checks exit 1 with `fmt-check PASS` + `check FAIL 101` (two compile errors) + `test FAIL` + `clippy FAIL`.
- Working tree NOT reverted on discovery — resume applies the amendment on top (§6).

## 0. Scope boundary (binding)

This amendment reopens EXACTLY:
1. The two verified compile defects in the base plan §3.3 draft (§1 below).
2. The fmt-stability facts about `src/platform/windows.rs` and their resolution (§3 below).

Nothing else reopens. Any other failure at S4/S5 resume → same STOP rules as base plan §4/§7.

## 1. Defect 1 — E0425 `cannot find function 'image_dimensions'` (§3.3 draft, app_display.rs)

- Draft state: `image_dimensions` was defined INSIDE `impl CurrentImageDisplay` while `paint` calls it as a module-scope free fn around base-plan draft line L39.
- Base-plan intent (own words): "`image_dimensions` is a private free fn (1 arg)" — i.e. module scope.
- Resolution (DECIDED, no alternative): move the fn OUT of the impl block to module scope, placed between the impl block and the tests module. The `paint` call site stays UNCHANGED (`image_dimensions(decoded_image)` — no `Self::` prefix).

## 2. Defect 2 — E0277 `TextureHandle doesn't implement Debug` (§3.3 draft, app_display.rs)

- Draft state: final test line was `assert_eq!(display.image_texture, Some(uploaded_texture));` around base-plan draft line L105. `assert_eq!` requires `Debug`; `egui 0.37 TextureHandle` implements `PartialEq`/`Eq` but NOT `Debug` (base plan fact V4 said exactly that — the draft test contradicted its own fact).
- Resolution (DECIDED): replace the line with `assert!(display.image_texture == Some(uploaded_texture));`. Identity-equality pin preserved via `PartialEq` (a re-upload yields a NEW `TextureId` → not equal → test fails ⇒ "upload once per decode" still pinned). No Debug bound needed.

## 3. Defect 3 (accepted drift) — fmt-stability of `src/platform/windows.rs` (out-of-freeze fact, RESOLVED as authorization)

- Verified evidence from the implementer: after S3 `cargo fmt`, `git status` shows `M src/platform/windows.rs` — a 3-line rustfmt reflow of the `windows::Win32::Graphics::Imaging::{…}` import list ONLY (handles lines 12–16 of the current file; whitespace-only; zero semantic delta).
- Root cause (verified): `git show HEAD:src/platform/windows.rs | rustfmt --check --edition 2024` exits 1 with EXACTLY that reflow; pinned toolchain rustfmt 1.10.0-stable. HEAD's `windows.rs` blob was never fmt-stable: Task 3's last fmt normalization preceded the fix3/fix4 byte-exact replacements, and the post-fix4 rounds ran only `cargo check --locked --target …` (no fmt). Today's reflow is therefore REQUIRED for `fmt-check` to stay green; reverting it breaks S4 — keeping it would contradict the base plan's byte-freeze. Both directions but only ONE is viable.
- Resolution (DECIDED — recommended path taken): ACCEPT the reflowed `src/platform/windows.rs` and commit it as a SEVENTH file in the Task-4 single commit. Rationale: keeps `fmt-check` green; least-churn path (the reflow is ALREADY in the tree from S3 — no reverse edit work); no semantic delta anywhere.
- Freeze language amendment (replaces base plan §1.2-d last sentence and §8 bullet "no `src/platform/**` change of any byte"): `windows.rs` is NOT touched semantically by Task 4; only its fmt-restored import-list form (3-line whitespace reflow) is committed, as a SANCTIONED FORMATTING-RESTORATION DEVIATION with this root-cause record. Deviation is recorded in S7 summary and is read-with material for workflow steps 4.3 (adherence review) and 4.5b (Task-4 verification) — reviewers must not flag it as a scope violation.

## 4. FINAL `src/app_display.rs` — full-file content (REPLACES the §3.3 draft entirely; §3.1/§3.2/§3.4/§3.5/§3.6 stay UNCHANGED from the base plan)

Deliver exactly this file (LF endings, trailing newline, UTF-8 no BOM). Note: the second `ensure_current_image_texture` signature is single-line here at width ≤ 100 — S3 `cargo fmt` is re-run anyway per §6 step 2, so only semantic content is binding; rustfmt owns line breaks.

```rust
//! Display state and paint step of the central image surface: texture lifecycle
//! (one upload per successful decode) and the fit-scaled centered render.

use eframe::egui;

use crate::image_loading::DecodedImage;
use crate::scaling;

const CURRENT_IMAGE_TEXTURE_NAME: &str = "current_image";

/// Texture state of the image shown in the central surface. A successful open
/// flags a pending upload; the next paint uploads exactly once and clears it.
pub(crate) struct CurrentImageDisplay {
    image_texture: Option<egui::TextureHandle>,
    texture_upload_pending: bool,
}

impl CurrentImageDisplay {
    pub(crate) fn initial() -> Self {
        Self {
            image_texture: None,
            texture_upload_pending: false,
        }
    }

    /// A successful decode replaced the decoded payload: the next paint uploads it.
    pub(crate) fn accept_decoded_image(&mut self) {
        self.texture_upload_pending = true;
    }

    /// Paints the decoded image fit-scaled and centered on the given surface.
    pub(crate) fn paint(&mut self, panel_ui: &mut egui::Ui, decoded_image: &DecodedImage) {
        self.ensure_current_image_texture(panel_ui.ctx(), decoded_image);
        let Some(image_texture) = self.image_texture.as_ref() else {
            return;
        };
        let Some(display_size) = scaling::fitted_size(
            panel_ui.available_size(),
            image_dimensions(decoded_image),
        ) else {
            return;
        };
        panel_ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |image_ui| {
                image_ui.add(egui::Image::new(image_texture).fit_to_exact_size(display_size));
            },
        );
    }

    /// Uploads the decoded pixels as a texture only while a new decode is pending,
    /// replacing the previous handle by assignment (the dropped handle frees the
    /// old texture). Never runs more than once per successful decode.
    fn ensure_current_image_texture(&mut self, ctx: &egui::Context, decoded_image: &DecodedImage) {
        if !self.texture_upload_pending {
            return;
        }
        let image_size = [decoded_image.width as usize, decoded_image.height as usize];
        let color_image =
            egui::ColorImage::from_rgba_unmultiplied(image_size, &decoded_image.pixels);
        self.image_texture = Some(ctx.load_texture(
            CURRENT_IMAGE_TEXTURE_NAME,
            color_image,
            egui::TextureOptions::default(),
        ));
        self.texture_upload_pending = false;
    }
}

/// Image dimensions as `egui::Vec2`, shared by the one `fitted_size` call in `paint`.
fn image_dimensions(decoded_image: &DecodedImage) -> egui::Vec2 {
    egui::Vec2::new(decoded_image.width as f32, decoded_image.height as f32)
}

#[cfg(test)]
mod tests {
    use super::CurrentImageDisplay;
    use crate::image_loading::DecodedImage;
    use eframe::egui;

    #[test]
    fn initial_display_has_no_texture_and_no_pending_upload() {
        let display = CurrentImageDisplay::initial();
        assert!(display.image_texture.is_none());
        assert!(!display.texture_upload_pending);
    }

    #[test]
    fn successful_upload_happens_exactly_once_per_decoded_image() {
        let mut display = CurrentImageDisplay::initial();
        display.accept_decoded_image();
        assert!(display.texture_upload_pending);

        let decoded_image = DecodedImage {
            width: 2,
            height: 2,
            pixels: vec![0; 16],
        };
        let context = egui::Context::default();
        display.ensure_current_image_texture(&context, &decoded_image);
        assert!(display.image_texture.is_some());
        assert!(!display.texture_upload_pending);
        let uploaded_texture = display.image_texture.clone().expect("uploaded texture");

        display.ensure_current_image_texture(&context, &decoded_image);
        assert!(display.image_texture == Some(uploaded_texture));
    }
}
```

Deltas vs the base plan §3.3 draft: exactly the two defect fixes of §1/§2 above. The ONLY other changes: none.

## 5. Updated commit plan (replaces base plan §5 commit-1 file list)

Commit 1 — Task 4 implementation (SINGLE commit):
- Files EXACTLY (7): `src/main.rs`, `src/app.rs`, `src/app_display.rs`, `src/scaling.rs`, `src/image_loading.rs`, `.agent/project-structure.md`, `src/platform/windows.rs`
- Message EXACT (UNCHANGED): `Display decoded image fit-scaled in the viewer window`

Procedure: identical to base plan §5 (gitignore read → `git status` → stage ONLY the seven paths, NO `git add .` → re-run `git status` → commit → clean). `src/platform/windows.rs` is a MODIFIED tracked file in the stage set; no gitignored path enters. Base plan §6.2 mapping rows stay valid as written; the 4.5b reading additionally records the fmt-restoration deviation (§3) as sanctioned, per S7 summary.

## 6. Resume instructions (implementer — from the current tree state)

- S2 amendment. Replace `src/app_display.rs` with §4 content (host file tools, LF, trailing newline, UTF-8 no BOM). `windows.rs` reflow is ALREADY in the tree from S3 — LEAVE AS-IS (no reverse edit).
- S3 retry. `vm_status` first, then: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` — `timeoutMs: 120000` (normalizes the replaced file; EBUSY → retry identical ONCE → proceed per base §3).
- S4 retry. `vm_status`, then `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` — `timeoutMs: 1200000`. EXPECT: exit 0; per-check `PASS fmt-check exit=0`, `PASS check exit=0`, `PASS test exit=0`, `PASS clippy exit=0`; `test result: ok. 15 passed` (test arithmetic unchanged: 4 + 4 + 2 + 5 = 15); final line `ALL CHECKS PASSED`. New compile failure NOT among the two fixed defects → base §4 sanctioned-in-draft rules; anything structural → STOP + report.
- S5. Base §4 S5 verbatim: FIRST df-gate (`df -h`, STOP if < 3 GiB available), then ONE `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu` — `timeoutMs: 1800000`. Expected exit 0. Errors ONLY in drafted files (`app.rs`/`app_display.rs`/`scaling.rs`/`main.rs`/`image_loading.rs`) → at most ONE sanctioned in-draft fix round; `windows.rs` semantic error or anything else → STOP + report.
- S6. Single 7-file commit per §5 (7 paths, exact message, gitignore procedure) then workflow 4.6 checkbox flips per base §3.4 note.
- S7. Summary must record BOTH amendment fixes (defect 1 move-to-module-scope, defect 2 assert-eq→assert-eq-PartialEq) AND the windows.rs fmt-restoration deviation with its root cause (§3) — flagged for 4.3/4.5b read-with. No evidence files in the repo.

DECISION SUMMARY: fix the two §3.3 defects by moving `image_dimensions` to module scope (call site unchanged) and pinning texture identity with `assert!(… == Some(…));`; accept the verified 3-line windows.rs import-list reflow as a sanctioned formatting restoration committed as the 7th file of the unchanged-message Task-4 commit; resume at S2-replace → fmt → dev-checks (expect 15 passed) → df-gated GNU check → commit → summary.
