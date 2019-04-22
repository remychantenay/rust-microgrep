# microgrep
microgrep is a (very) naive grep implementation wrote in Rust.

Wrote for fun while giving a go at Rust.

**Not intended to be used in any shape or form.**

## Build
```bash
cargo build
```

## Run
```bash
cargo run <QUERY> <FILENAME>
```

### Example
```bash
cargo run banish poem.txt
cargo run bANiSh poem.txt --i
```

## Unit Tests

```bash
cargo test
```