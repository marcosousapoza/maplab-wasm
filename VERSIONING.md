# Versioning and npm Releases

Maplab WASM follows [Semantic Versioning](https://semver.org/). `Cargo.toml` is the
source of truth. The generated npm package uses the same version.

## Compatibility

- Patch: numerical fixes that preserve exported names and argument semantics.
- Minor: backward-compatible exported functions.
- Major: removed exports or incompatible JavaScript/WASM behavior.

## Bump

```bash
./scripts/bump-version.sh patch  # or minor / major
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
git add Cargo.toml Cargo.lock
git commit -m "Bump WASM version to X.Y.Z"
```

## Publish to npmjs

Push a `vX.Y.Z` tag matching `Cargo.toml`:

```bash
git tag vX.Y.Z
git push origin main vX.Y.Z
```

`.github/workflows/publish-package.yml` verifies the tag, runs all checks, builds
with `wasm-pack --scope marcosousapoza`, and publishes
`@marcosousapoza/maplab-wasm` publicly to npmjs. npm trusted publishing exchanges
the GitHub Actions OIDC identity for a short-lived publishing credential and adds
package provenance; no npm token is stored in GitHub.

```bash
npm install @marcosousapoza/maplab-wasm@X.Y.Z
```
