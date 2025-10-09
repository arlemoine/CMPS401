## Why Rust?

- Lot of resources due to popularity.
- Balances computational efficiency with safety, utilizing the many benefits of C/C++ with additional guardrails.
- Compiled language
- Memory access checked during compilation, saving computation time.
- Rust is used to build web servers, creating games, operating systems, and much more!

## Setup

Further setup instructions and information can be found at the rust [website](https://www.rust-lang.org/learn/get-started).
In your Linux terminal, perform the following and follow the instructions.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Cargo

Cargo is the default build tool and package manager. It can do the following:
- Build project via `cargo build`
- Run project via `cargo run`
- Test project via `cargo test`
- Build documentation for your project via `cargo doc`
- Publish a library to [crates.io](http://crates.io) via `cargo publish`

For more information, refer to the [Cargo Book](https://doc.rust-lang.org/cargo/index.html).

## Testing Sample Programs

Prerequisite: `cargo` is installed on your operating system.

To test a sample program, go into the project directory (such as `/Samples/user_input/`) and use the following command. This command both compiles and runs the program.

```bash
cargo run
```
