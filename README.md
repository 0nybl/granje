# granje

Starter template for `0nybl` Rust projects. Created via **Use this template**.

Ships with the fleet baseline already satisfied:
- `.github/workflows/ci.yml` — `cargo test` on push/PR
- `.github/workflows/eerk.yml` — weekly baseline/compliance check (`0eerk[bot]`)
- `rustfmt.toml`, `LICENSE` (MIT), `.gitignore`

## After spawning a new repo
1. Rename the crate in `Cargo.toml` (`name = "..."`) to match the repo.
2. Replace `src/lib.rs` with real code.
3. Fill in the crate `description`.

The eerk workflow self-checks this repo against `0nybl/eerk-baseline` and
reports drift to a `repo-drift` tracking issue.
