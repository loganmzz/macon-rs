# Changelog

Source: https://keepachangelog.com/

## [Unreleased]

### Added

- Support crate configuration (#25)

## [1.0.0] - 2024-02-02

* **Crate**: https://crates.io/crates/macon/1.0.0
* **Documentation**: https://docs.rs/macon/1.0.0/macon/

### Added

- Tuple support (#4)
- Setters are generic over [`Into`](https://doc.rust-lang.org/core/convert/trait.Into.html) (#9)
- Builders implement [`Into`](https://doc.rust-lang.org/core/convert/trait.Into.html) (#11)
- [`Default`](https://doc.rust-lang.org/core/default/trait.Default.html) support for struct (#15)
- [`Default`](https://doc.rust-lang.org/core/default/trait.Default.html) support for fields (#14)

### Changed

- Improved documentation (#5)
- Remove mutable borrows for Panic/Result builder functions (#7)
- [`Option`](https://doc.rust-lang.org/core/option/enum.Option.html) fields are not mandatory anymore (#13)
- [`Option`](https://doc.rust-lang.org/core/option/enum.Option.html) field setters used wrapped type (#13)

## [0.2.0] - 2023-12-11

* **Crate**: https://crates.io/crates/macon/0.2.0
* **Documentation**: https://docs.rs/macon/0.2.0/macon/

This is the first official release.
