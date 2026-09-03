# Releasing AlgoBuddy

AlgoBuddy develops on `dev`; `main` is the production branch used by GitHub
Pages. A release is promoted through a pull request rather than developed
directly on `main`.

## Release Candidate

1. Freeze unrelated feature work and resolve every release blocker.
2. Update `CHANGELOG.md` with a dated version heading.
3. Set the same version in `Cargo.toml` and the AlgoBuddy package entry in
   `Cargo.lock`.
4. Commit all candidate files, including new and renamed tests.
5. From a clean tree, run:

   ```powershell
   ./scripts/release-check.ps1 -ExpectedVersion X.Y.Z -RequireClean
   ```

6. Run the browser launch test documented in `CONTRIBUTING.md`, then smoke-test
   the native application. Verify navigation, custom input, Play/Pause,
   Prev/Next, scrub, Reset, and every visualizer repaired by the release.
7. Push `dev` and wait for CI and CodeQL to pass on the candidate commit.

## Publish

1. Open a release pull request from `dev` to `main` using the changelog as its
   summary.
2. Merge only after required checks pass.
3. Verify `CI & Quality Gates` completed its native gates, launched the local
   WebAssembly bundle, deployed that tested artifact, and launched the published
   GitHub Pages site at the expected commit revision.
4. Create and push annotated tag `vX.Y.Z` on the exact merged `main` commit.
5. Verify that the tag workflow publishes a GitHub Release from the matching
   changelog section.
6. Open the next contributor issue batch after the released templates and
   contribution guide are live.

Do not tag a dirty tree, tag a commit that differs from the deployed `main`, or
publish a release when the Cargo and changelog versions disagree.
