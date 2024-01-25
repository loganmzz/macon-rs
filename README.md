# macon-rs

![Macon logo](./assets/logo-200x184.png)

Another builder macro-based generator with its own idioms.

## Origin

"[Maçon](https://fr.wiktionary.org/wiki/ma%C3%A7on#Nom_commun_2)" (pronounced: [\\ma.sɔ̃\\](https://commons.wikimedia.org/wiki/File:LL-Q150_(fra)-Fhala.K-ma%C3%A7on.wav)) is French translation for "[builder](https://www.wordreference.com/enfr/builder)".

Idea for creating a new builder generator comes from benchmarking solutions at: https://github.com/loganmzz/rust-benchmark-setter.

## Usage

[`macon` crate](https://crates.io/crates/macon) provides a derive macro:

```rust
#[macro_use] extern crate macon;

#[derive(Builder)]
struct MyType {
  integer: i32,
  string: String,
  optional: Option<String>,
}

let _mytype: MyType = MyType::builder()
    .integer(42)
    .string("foobar")
    .build();

#[derive(Builder)]
struct MyTuple(
  i32,
  String,
  Option<String>,
);

let _mytuple: MyTuple = MyTuple::builder()
    .set(42)
    .set("foobar")
    .build();
```

See [crate documentation](https://docs.rs/macon/latest/macon/) for more information about available options.

## Version history

See [CHANGELOG.md](./CHANGELOG.md)

## Development

### Toolchain

Toolchain version is left unspecified, only edition is set. Let's your [`rustup`](https://rustup.rs/) shims manage it!

### Show expansion on test

The `tests/*.rs` are good way to see how to use and debug macro. Consider expanding test files with: `cargo expand --test <test basename>`. Example: `cargo expand --test 'mode_panic'`.

### Open documenation

```bash
cargo doc --document-private-items --no-deps --open
```

### Release

```bash
cargo run -p macon_tools --bin release <version>
```

Then follow instructions.
