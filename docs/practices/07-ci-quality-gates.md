# 07. CI Quality Gates

## Required gates

Every pull request or agent change MUST pass these gates:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo llvm-cov --workspace --all-features --fail-under-lines 80 --show-missing-lines
```

Also enforce the 100-line Rust source-file limit.

## File length gate

Add this check to CI or an `xtask` equivalent:

```sh
violations="$({ find src tests examples benches -name '*.rs' -type f 2>/dev/null || true; } \
  | while read -r file; do
      lines=$(wc -l < "$file")
      if [ "$lines" -gt 100 ]; then
        echo "$file:$lines"
      fi
    done)"

if [ -n "$violations" ]; then
  echo "Rust source files over 100 lines:"
  echo "$violations"
  exit 1
fi
```

## Example GitHub Actions job

```yaml
name: rust-quality

on:
  pull_request:
  push:
    branches: [main]

jobs:
  quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy, llvm-tools-preview
      - uses: taiki-e/install-action@cargo-llvm-cov
      - name: Format
        run: cargo fmt --all -- --check
      - name: Lint
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings
      - name: Test
        run: cargo test --workspace --all-features
      - name: Coverage
        run: cargo llvm-cov --workspace --all-features --fail-under-lines 80
      - name: Source file length
        run: |
          violations="$({ find src tests examples benches -name '*.rs' -type f 2>/dev/null || true; } \
            | while read -r file; do
                lines=$(wc -l < "$file")
                if [ "$lines" -gt 100 ]; then
                  echo "$file:$lines"
                fi
              done)"
          if [ -n "$violations" ]; then
            echo "Rust source files over 100 lines:"
            echo "$violations"
            exit 1
          fi
```

## Recommended local alias

```sh
cargo fmt --all \
  && cargo clippy --workspace --all-targets --all-features -- -D warnings \
  && cargo test --workspace --all-features \
  && cargo llvm-cov --workspace --all-features --fail-under-lines 80
```

## Failure policy

- Formatting failure: run formatter and review diff.
- Clippy failure: fix the code; local `allow` requires a reason.
- Test failure: fix behaviour or update invalid tests.
- Coverage failure: add meaningful tests or remove dead code.
- File length failure: split by domain responsibility.
