# Rust World Hello Demo

## Build Rust Env

Install rustup from <https://rustup.rs/>.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup self uninstall

# check version
$ rustc -V
$ cargo -V
```

Create project:

```sh
# new bin crate
cargo new world_hello
# new lib crate
cargo new world_hello --lib
```

Create a lib crate `hello_macro_derive` for custom macro:

```sh
# 宏所在的包必须以 derive 为后缀
cargo new hello_macro_derive --lib
```

Build and run:

```sh
cargo check
cargo build
cargo run
```

Run test:

```sh
# only build
cargo test --no-run

cargo test
cargo test -- --show-output
```

