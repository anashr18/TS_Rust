
# 🦀 Rust CLI Navigation Cheat Sheet (with ripgrep, bat, eza, fd)

This cheat sheet helps you navigate and understand open-source Rust projects like `mini-redis`, `axum`, and others using powerful CLI tools.

---

## 🔍 ripgrep (rg)

| Goal                                  | Command                                  |
|---------------------------------------|-------------------------------------------|
| Find all function definitions         | `rg '^fn '`                               |
| Find a specific function              | `rg '^fn my_function'`                    |
| Find function usage                   | `rg 'my_function'`                        |
| Search by trait implementations       | `rg 'impl MyTrait'` or `rg 'impl '`       |
| Trace enum definitions                | `rg '^pub enum '`                         |
| Find where enum variants are matched  | `rg 'match .*::MyEnum'`                  |
| Search for struct definitions         | `rg '^pub struct '`                       |
| Find module declarations              | `rg '^mod '`                              |
| Find all async functions              | `rg '^async fn '`                         |
| Find test functions                   | `rg '#\[test\]'`                        |
| Trace macro usages                    | `rg 'macro_rules!'` or `rg 'println!'`    |
| Trace crate imports                   | `rg '^use '`                              |
| Trace return types with Result        | `rg '-> Result<'`                         |
| Trace return types with Future        | `rg '-> impl Future'`                     |
| Find all match statements             | `rg '^ *match '`                          |
| Find all trait definitions            | `rg '^pub trait '`                        |

---

## 🗂️ File & Project Navigation

| Tool     | Command                            | Description                        |
|----------|------------------------------------|------------------------------------|
| fd       | `fd main.rs`                       | Find files faster than `find`      |
| bat      | `batcat src/main.rs`                  | View file with syntax highlighting |
| eza      | `eza -T -L 2`                      | Tree view of folders               |
| tree     | `tree -L 2` (if no eza)            | Traditional tree view              |
| grep     | `grep -R "tokio::spawn" .`         | Fallback to grep if no rg          |
| fzf      | `rg . | fzf`                       | Fuzzy find anything in project     |

---

## 📦 Cargo Utilities

| Goal                                 | Command                         |
|--------------------------------------|----------------------------------|
| View dependency tree                 | `cargo tree`                    |
| View metadata                        | `cargo metadata --no-deps`     |
| Run tests                            | `cargo test`                   |
| Run specific test                    | `cargo test test_name`         |
| Format code                          | `cargo fmt`                    |
| Check for warnings/errors            | `cargo check`                  |
| Build project                        | `cargo build`                  |
| Clean build artifacts                | `cargo clean`                  |

---

## 🧪 Rust Testing Commands

| Type                  | Command / Pattern                      |
|-----------------------|-----------------------------------------|
| Unit Test             | Inside `mod tests {}` with `#[test]`    |
| Integration Test      | In `tests/` folder, uses full crate     |
| Property-Based Test   | Use `proptest!` macro                   |
| Run All Tests         | `cargo test`                            |
| Run Integration Only  | `cargo test --test <file_name>`         |
| Test Coverage         | Use `cargo tarpaulin` or `grcov`        |

---

## 🔗 Good to Know

- Most Rust open source repos follow this layout:
  - `src/lib.rs`, `src/main.rs`
  - `mod`, `pub mod` map to file structure
  - `tests/` folder for integration tests
  - `examples/` for sample usage
- Use `rg` to trace how a `Frame`, `State`, or `Router` is used across modules

---

Happy Hacking 🦀🔥
