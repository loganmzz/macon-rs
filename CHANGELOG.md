# Changelog

Source: https://keepachangelog.com/

## 0.3.0

* **Crate**: https://crates.io/crates/macon/0.3.0
* **Documentation**: https://docs.rs/macon/0.3.0/macon/

### Added

- Tuple support
- Setters are generic over [`Into`](https://doc.rust-lang.org/core/convert/trait.Into.html)
- Builders implement [`Into`](https://doc.rust-lang.org/core/convert/trait.Into.html)
- [`Default`](https://doc.rust-lang.org/core/default/trait.Default.html) support for struct

### Changed

- Improved documentation
- Remove mutable borrows for Panic/Result builder functions
- [`Option`](https://doc.rust-lang.org/core/option/enum.Option.html) fields are not mandatory anymore
- [`Option`](https://doc.rust-lang.org/core/option/enum.Option.html) field setters used wrapped type

## 0.2.0

* **Crate**: https://crates.io/crates/macon/0.2.0
* **Documentation**: https://docs.rs/macon/0.2.0/macon/

This is the first official release.
