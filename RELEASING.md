# Release process

1. Ensure tests are passing with `cargo test` and check CI
2. Verify `CHANGELOG.md` is up to date
3. Update version number in `Cargo.toml`, `README.md`, `CHANGELOG.md` (and commit changes)
4. Run `cargo publish`
5. `git tag -a ...` with version number
6. `git push` and `git push --tags`
