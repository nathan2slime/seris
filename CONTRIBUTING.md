# Contributing to Seris

Seris keeps changes small, explicit, and easy to verify.

## Workflow

1. Branch from `master`.
2. Keep the branch focused on one issue or one small fix.
3. Run formatting, tests, and linting before opening a PR.

## Local checks

```bash
cargo fmt --check
cargo test --locked --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
```

## Guidelines

* Prefer the smallest correct change.
* Keep public API and docs in sync.
* Add tests for new behavior whenever practical.
* Avoid introducing new dependencies unless they clearly reduce complexity.

## PRs

* Reference the issue number in the branch and PR title when possible.
* Summarize the behavior change, verification steps, and any operational impact.
* Call out config, deployment, or monitoring changes explicitly.
