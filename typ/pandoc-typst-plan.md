# Pandoc / Typst Book Plan

## Goal

Add a second book-generation path for HTML and PDF while keeping the current mdBook site intact.

## Decisions

- Keep `bk/src` as the single source of truth for book content.
- Keep mdBook as the canonical HTML pipeline.
- Add a PDF/export pipeline incrementally instead of replacing mdBook outright.
- Treat the existing `typ/typst/` project as a design and evaluation sandbox, not as a second authoring source.

## Why This Direction

The repository already contains most of the plumbing for a Pandoc-based export path:

- `bk/book.toml` already defines an `output.pandoc` section with a PDF profile.
- The dev container already installs `pandoc`, `mdbook-pandoc`, and `tectonic`.
- The current local recipes and CI entrypoints still build only the mdBook HTML output.

That makes Pandoc the lowest-risk first step. Typst is still worth evaluating, but only after there is a working secondary export path from the existing Markdown sources.

## Recommended Sequence

### Phase 1: Ship a Supported PDF Path

1. Keep the current mdBook HTML flow unchanged.
2. Choose a small representative chapter set for validation.
3. Activate the existing Pandoc PDF backend in the contributor workflow.
4. Add explicit local build commands for PDF output.
5. Upload the PDF as a CI artifact without changing GitHub Pages deployment.
6. Document the dual-pipeline workflow.

### Phase 2: Run a Typst Spike

1. Evaluate `mdbook-typst` against the same representative chapter set.
2. Check whether it preserves the repo's current preprocessing behavior.
3. Reuse ideas from `typ/typst/` for styling only.
4. Decide whether Typst becomes a supported target or remains experimental.

### Phase 3: Only If Typst Proves Worthwhile

1. Install `typst` in the dev and CI environments.
2. Add local Typst export recipes.
3. Add CI artifacts for Typst output.
4. Document which backend is used for which output format.

## Work Plan

### 1. Baseline the Current Pipeline

Use the existing mdBook stack in `bk/book.toml` as the compatibility baseline:

- `private`
- `scrub`
- `index`
- `links`
- `indexing`

Before adding any new backend, select sample chapters that exercise:

- `{{#include}}` expansions
- centralized reference definitions
- code fences and hidden lines
- tables
- hidden/private content

This sample set becomes the acceptance test for Pandoc and Typst output.

### 2. Add a First-Class Pandoc Export

The repository already has a usable starting point in `bk/book.toml`:

- `[output.pandoc]`
- `[output.pandoc.code]`
- `[output.pandoc.profile.pdf]`

Implementation tasks:

1. Add explicit PDF build recipes in `bk/scripts/book/mod.just`.
2. Expose those recipes through the usual `just` entrypoints.
3. Keep the existing HTML build and serve commands unchanged.
4. Decide on the output location and artifact naming convention.

Expected result:

- contributors can run a stable command to produce a PDF from the existing Markdown book
- HTML remains owned by mdBook

### 3. Wire PDF Output Into CI

The CI build should remain HTML-first, with PDF added as an artifact:

1. Extend `.devcontainer/ci.sh` to run the PDF export after the HTML build.
2. Update `.github/workflows/build_deploy_book.yml` to upload the PDF artifact.
3. Continue deploying only `book/html/` to GitHub Pages.

This keeps the public site unchanged while making the new export path visible and testable.

### 4. Document the Supported Workflow

Update project docs so the intended model is explicit:

- Markdown in `bk/src` is the only source of truth.
- mdBook remains the web/HTML pipeline.
- Pandoc is the supported print/PDF export path.
- Typst remains experimental until proven viable.

This avoids drift between the Markdown book and the separate Typst prototype.

## Detailed Task List

### Phase 1: Pandoc PDF Path

1. Audit current build and CI flow.
   - Confirm `mdbook build` currently produces HTML only.
   - Verify `bk/book.toml` has a valid `output.pandoc` profile.
   - Confirm the dev container and CI image already install `pandoc`, `mdbook-pandoc`, and `tectonic`.
2. Choose representative validation chapters.
   - Include chapters that use `{{#include}}`, `refs.incl.md`, tables, code blocks with hidden lines, and private sections.
   - Document the chosen chapter list in this plan or a companion note.
3. Add local build commands for PDF output.
   - Add a `pdf` or `export-pdf` recipe to `bk/scripts/book/mod.just`.
   - Add an alias in `bk/justfile` or root `justfile` if needed.
   - Keep existing `html` and `serve` rules unchanged.
4. Validate the Pandoc build locally.
   - Run the new recipe against the sample chapters.
   - Confirm output location, filename, and build reproducibility.
5. Add CI artifact generation.
   - Extend `.devcontainer/ci.sh` to run the PDF build step after HTML.
   - Update `.github/workflows/build_deploy_book.yml` to upload the produced PDF artifact.
   - Ensure `book/html/` remains the only deployment source for Pages.
6. Document the new workflow.
   - Update `README.md` with `just` commands for PDF export.
   - Update `typ/README.md` to describe the plan and clarify that Typst is experimental.
   - Optionally add a short note in `typ/pandoc-typst-plan.md` referencing the final commands.

### Phase 2: Typst Evaluation

1. Review Typst project status.
   - Confirm `typ/typst/main.typ` is a template reference, not a synced source.
   - Confirm the Typst files are usable as style/layout examples.
2. Install or enable `mdbook-typst` for testing.
   - Prefer `cargo install mdbook-typst` in a local dev environment first.
   - Avoid changing CI until the spike proves viable.
3. Run a Typst export against the representative chapters.
   - Use the same sample set selected in Phase 1.
   - Compare the exported result with the Pandoc PDF output.
   - Verify inclusion of references, code blocks, and hidden content behavior.
4. Evaluate results.
   - Check whether Typst preserves existing preprocessing and markup.
   - Evaluate output quality versus the Pandoc PDF.
   - Document any gaps, required workarounds, or custom filters.
5. Decide whether to adopt Typst.
   - If yes, proceed to Phase 3.
   - If no, keep Typst as experimental and maintain the Pandoc path.

### Phase 3: Typst Adoption (Optional)

1. Add Typst toolchain installation.
   - Update `.devcontainer/Dockerfile` and `.devcontainer/install_deps.sh` if Typst is supported.
   - Optionally add `typst` installation instructions to `.devcontainer/.env`.
2. Add Typst export recipes.
   - Add `typst` or `export-typst` recipes to `bk/scripts/book/mod.just`.
   - Add documentation for the new command.
3. Add CI artifact upload.
   - Upload Typst-generated PDF/artifacts in GitHub Actions.
   - Keep Pages deployment unchanged.
4. Update documentation.
   - Document which backend is used for HTML, PDF, and any future print output.
   - Clarify the role of `typ/typst/` as template guidance.

## Typst Evaluation Criteria

Evaluate Typst only after the Pandoc path exists.

Questions to answer:

1. Can `mdbook-typst` consume the mdBook-preprocessed content cleanly?
2. Does it preserve links, includes, headings, and code blocks well enough?
3. Does it materially improve print quality compared with Pandoc plus LaTeX/Tectonic?
4. Is the maintenance cost acceptable?

If the answer is mostly yes, add Typst as a supported backend.
If not, keep Typst as a sandbox for future print-only experiments.

## Role of `typ/typst/`

The current Typst project is useful, but it should not become the primary content source by default.

Use it for:

- page layout ideas
- front matter structure
- appendix structure
- bibliography setup
- listing and figure presentation

Do not use it for:

- manually duplicating the whole book
- maintaining a second prose source tree in parallel with `bk/src`

## Files Likely To Change

- `bk/book.toml`
- `bk/scripts/book/mod.just`
- `bk/justfile`
- `.devcontainer/ci.sh`
- `.github/workflows/build_deploy_book.yml`
- `README.md`
- `typ/README.md`

## Verification

### Local Verification

1. Existing HTML build still works unchanged.
2. New PDF build produces a deterministic artifact.
3. Representative chapters render correctly in both HTML and PDF.

### Content Verification

Check these specifically:

- include expansion
- centralized refs and links
- tables
- code blocks
- heading numbering
- hidden content behavior

### CI Verification

1. Pages deployment still publishes only `book/html/`.
2. PDF is uploaded as a downloadable artifact.
3. No new backend requirement is added to contributors unless it is documented.

## Recommendation

Recommended order of execution:

1. Ship Pandoc PDF first.
2. Test and document it.
3. Run a bounded Typst spike.
4. Adopt Typst only if it clearly improves output without adding a second source of truth.

This delivers useful output quickly and keeps the book architecture simple.
