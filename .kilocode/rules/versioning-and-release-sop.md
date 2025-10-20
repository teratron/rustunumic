# Semantic Versioning and Release SOP

## Brief overview

This document establishes a strict, repeatable standard operating procedure (SOP) for versioning the `rustunumic` project, ensuring consistency across all project artifacts and clear communication of changes. This rule must be followed for any changes intended for a public release.

## Trigger

This procedure is executed as the final step after all development, feature integration, bug fixes, and quality assurance checks (per `.kilocode/rules/test-driven-workflow.md`) are complete and the main branch is ready for a new release.

## Procedure

1. **Determine Version Increment**: Analyze the accumulated changes since the last release and determine the new version number strictly following the Semantic Versioning (SemVer) 2.0.0 specification:
    * **MAJOR (X.y.z):** Increment for backward-incompatible API changes.
    * **MINOR (x.Y.z):** Increment for adding new, backward-compatible functionality.
    * **PATCH (x.y.Z):** Increment for making backward-compatible bug fixes.

2. **Update Version String**: Systematically update the project's version number to the newly determined version in all designated files. The update must be consistent across:
    * `Cargo.toml` (specifically in the `[package]` section's `version` field)
    * `README.md` (update any version badges or installation examples)

3. **Finalize Changelog**: Update the `CHANGELOG.md` file to reflect the new release:
    * Create a new heading for the new version (e.g., `## [vX.Y.Z] - YYYY-MM-DD`).
    * Ensure all changes for the release are documented under this heading, categorized by `Added`, `Changed`, `Fixed`, or `Removed`.
    * Ensure the `[Unreleased]` section is now empty or contains only changes for the *next* release cycle.

4. **Create Release Commit**: Stage all modified files and create a single, dedicated commit for the version bump. The commit message should follow a conventional format.
    * **Command:** `git add Cargo.toml README.md CHANGELOG.md docs/`
    * **Command:** `git commit -m "chore(release): bump version to vX.Y.Z"`

5. **Create Annotated Git Tag**: Tag the release commit with the corresponding version number. The tag must be annotated with a release summary.
    * **Command:** `git tag -a vX.Y.Z -m "Release vX.Y.Z"`

6. **Push to Remote**: Push the release commit and the new tag to the `main` (or `master`) branch of the remote repository to finalize the release.
    * **Command:** `git push`
    * **Command:** `git push origin vX.Y.Z`
