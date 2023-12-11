# macon-rs

Another builder macro-based generator with its own idioms.

"[Maçon](https://fr.wiktionary.org/wiki/ma%C3%A7on#Nom_commun_2)" is French translation for "[builder](https://www.wordreference.com/enfr/builder)"

## Development

### Toolchain

Toolchain version is left unspecified, only edition is set. Let's your [`rustup`](https://rustup.rs/) shims manage it !

### Show expansion on test

The `tests/*.rs` are good way to see how to use and debug macro. Consider expanding test files with: `cargo expand --test <test basename>`. Example: `cargo expand --test 'mode_panic'`.
