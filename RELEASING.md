# Releasing

All three crates (`sus_compiler`, `sus-proc-macro`, `tree-sitter-sus`) share one version and are released together.

1. In the root `Cargo.toml`, set the version in `[workspace.package]` and in both `[workspace.dependencies]` entries (drop the `-dev` suffix).
2. In `CHANGELOG.md`, add a `## X.Y.Z: Title` section at the top. It becomes the GitHub release title and notes.
3. `cargo build`, so `Cargo.lock` picks up the new version.
4. Commit, tag and push:
   ```sh
   git commit -am "Release X.Y.Z :tada:"
   git tag vX.Y.Z
   git push origin master vX.Y.Z
   ```
5. `.github/workflows/release.yml` takes over: it checks that the tag matches the Cargo version and the changelog, runs CI, runs `cargo publish --workspace --locked`, and creates the GitHub release.
6. Afterwards, bump the three versions to `X.Y.(Z+1)-dev`.

To check locally beforehand: `cargo publish --workspace --locked --dry-run`

## One-time setup

The workflow uses crates.io Trusted Publishing. For each of the three crates: crates.io → Settings → Trusted Publishing → repository `pc2/sus-compiler`, workflow `release.yml`, environment `release`.
