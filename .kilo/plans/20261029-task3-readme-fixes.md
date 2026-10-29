# Task 3 README Fixes

## Deviations found in README.md

### 1. Technology Stack table Notes column not identical to tech.md

Per the approved implementation plan (Section 4.5), the tech table rows must use values identical to `.agent/project-info/tech.md` "Technology Stack (Planned)". Current `README.md` deviates in four rows.

Fix: update the Notes cells to match exactly.

| Component | Current | Required |
| --- | --- | --- |
| Language | Confirmed decision; not yet implemented. | Confirmed decision in the brief; not yet implemented. |
| Image decoding | Accepts any image format registered in the system instead of bundling decoders. | The app accepts any image format registered in the system instead of bundling its own decoders. |
| Target platform | Planned. | Planned (not yet implemented). |
| Target architectures | Planned. | Planned (not yet implemented). |

### 2. Provisional-name note not immediately under title

The plan (Section 4.1) requires the italic provisional-name note "immediately under the title". `README.md` has a blank line between `# img-scroller` and the italic line.

Fix: remove the blank line so the italic line follows the title directly.

## Files to touch

| File | Action |
| --- | --- |
| `README.md` | Apply the two fixes above; no other changes. |

## Verification

- [ ] Tech table Notes match `tech.md` exactly.
- [ ] Title and provisional-name note are consecutive lines.
- [ ] No other `README.md` content changed.
