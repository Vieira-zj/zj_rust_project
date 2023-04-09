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
# new bin project
cargo new world_hello
# new lib
cargo new world_hello --lib
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

Create a lib `hello_macro_derive` for custom macro demo:

```sh
cargo new hello_macro_derive --lib
```

## Project Structure

Refer: <https://github.com/ellie/atuin>

