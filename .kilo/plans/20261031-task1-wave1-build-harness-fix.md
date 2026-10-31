# Amendment — Task 1 Wave 1 Build Harness: fix docker-compose.yml volume reference

- Date: 2026-10-31
- Amends: [`20261031-task1-wave1-build-harness.md`](20261031-task1-wave1-build-harness.md) §2.2 (`docker-compose.yml` draft) — the ONLY change this amendment authorizes. The base plan file itself is NOT edited (repo rule: new plan files only).
- Why: implementer evidence (deterministic, exit 15 twice): the §2.2 draft's `rust` service volume mount referenced `img-scroller-cargo-target:/cargo-target` (the volume NAME) while the top-level `volumes:` block declares the volume under key `cargo-target` with `name: img-scroller-cargo-target`. Docker Compose requires the service to reference the KEY, not the name → `service "rust" refers to undefined volume img-scroller-cargo-target: invalid compose project`.

## Decision

**Option (a)** is chosen: keep top-level key `cargo-target` with `name: img-scroller-cargo-target` and change ONLY the service mount line to `- cargo-target:/cargo-target`.

Rationale: the two options are not fully equivalent. Under option (b) (rename the key to `img-scroller-cargo-target`, drop `name:`), Compose v2 assigns a managed top-level volume without an explicit `name:` attribute the project-prefixed name `<project>_<key>` (project = `img-scroller`, derived from `/img-scroller`) → actual volume name `img-scroller_img-scroller-cargo-target`, violating the base plan's decision 1.b. Only the explicit `name:` attribute pins the exact volume name independent of the project name. Option (a) is also the minimal one-line diff.

**Resulting actual Docker volume name after the fix: `img-scroller-cargo-target`** (explicit `name:` attribute wins over any project prefix). Decision 1.b (base plan §1.1 row b) is preserved exactly.

## EXACT final `docker-compose.yml` (replace the working-tree file verbatim)

```yaml
services:
  rust:
    image: img-scroller-rust:latest
    build:
      context: .
      dockerfile: Dockerfile
    working_dir: /img-scroller
    volumes:
      - /img-scroller:/img-scroller
      - cargo-target:/cargo-target
    environment:
      CARGO_TARGET_DIR: /cargo-target

volumes:
  cargo-target:
    name: img-scroller-cargo-target
```

File requirements unchanged: new file at repository root, LF line endings, no BOM, trailing newline.

Semantics note replacing the corresponding §2.2 bullet: the named volume is declared under the top-level key `cargo-target` with the explicit `name: img-scroller-cargo-target`; the service references the KEY (`cargo-target`), and the explicit `name:` attribute makes the actual managed volume name exactly `img-scroller-cargo-target` (no project prefix). All other §2.2 semantics (bind mount, `working_dir`, `CARGO_TARGET_DIR=/cargo-target`, `image:` + `build:` tag, no `version:` key, relative `context: .`) are unchanged.

## No other plan deliverable changes

- `Dockerfile` (§2.1), `scripts/dev-checks.sh` (§2.3), `scripts/build-windows.sh` (§2.4), `docs/build.md` (§2.5), `docs/build-windows.md` (§2.6), `.gitignore` edit (§2.7), `.agent/project-structure.md` edit (§2.8): all as drafted; no change.
- Volume-name decision 1.b (`img-scroller-cargo-target`), mount path `/cargo-target`, `CARGO_TARGET_DIR=/cargo-target`, docs prefixes and all other decisions: unchanged.
- Base plan §6 verification mapping: the row `docker-compose.yml (service, mounts, volume, CARGO_TARGET_DIR)` must be read with the amendment applied — pass state = file matches §2.2 **as amended by this file** (service mount key `cargo-target`; explicit `name: img-scroller-cargo-target`; actual volume name `img-scroller-cargo-target`). No other §6 row is affected.

## Resume instructions (implementer, workflow §4.2)

1. Replace the working-tree `docker-compose.yml` with the exact YAML above (write tool; LF, no BOM, trailing newline).
2. Continue at base-plan S5 (image build, retry policy unchanged): `vm_status` first, then `docker compose -f /img-scroller/docker-compose.yml build rust`, `timeoutMs: 900000`; on persistent failure → STOP + escalate with captured output.
3. Then S6 (toolchain sanity) → S7 (`sh -n` syntax checks, two calls) → S8 (git hygiene + commits per §4 exactly).
4. Commit plan unchanged: the compose file was NOT yet committed, so Commit 1 (`Wire container build harness: pinned Rust image, dev-checks, Windows x64 cross-build`) still includes `docker-compose.yml` among its listed files; Commit 2 (docs) unchanged.
5. Record in the 4.2 summary that the compose file was written from this amendment (not base §2.2 verbatim).
