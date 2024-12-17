# Changelog

Source: https://keepachangelog.com/

## [Unreleased]

## [1.3.0] - 2024-12-17

* **Crate**: https://crates.io/crates/macon/1.3.0
* **Documentation**: https://docs.rs/macon/1.3.0/macon/

### Added

- Add `<field>_optional` setter with `Option<>` parameter for optional fields (#34)
- Document `Builder` signature (#34)

## [1.2.0] - 2024-09-15

* **Crate**: https://crates.io/crates/macon/1.2.0
* **Documentation**: https://docs.rs/macon/1.2.0/macon/

### Deprecated

- Deprecate `Option=!` and `Into=!` for struct attribute `#[builder]` in favor of `fields(Option=!)` and `fields(Into=!)` (#32)

### Added

- Add nested `fields(...)` value for struct attribute `#[builder]` (#32)
- Add `fields(Default=!)` for struct attribute `#[builder]` (#32)

## [1.1.0] - 2024-08-26

* **Crate**: https://crates.io/crates/macon/1.1.0
* **Documentation**: https://docs.rs/macon/1.1.0/macon/

### Added

- Add setting to disable `Into` support for setters (#30)

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
