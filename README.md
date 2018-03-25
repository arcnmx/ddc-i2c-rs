# ddc-i2c

[![travis-badge][]][travis] [![release-badge][]][cargo] [![docs-badge][]][docs] [![license-badge][]][license]

`ddc-i2c` implements the [`ddc`](https://crates.io/crates/ddc) traits for
[`i2c`](https://crates.io/crates/i2c) implementations.

## Backends

- `i2c-linux` using the `with-linux` Cargo feature.
  - The `with-linux-enumerate` feature exposes an iterator over all detected
    displays.

## [Documentation][docs]

See the [documentation][docs] for up to date information.

[travis-badge]: https://img.shields.io/travis/arcnmx/ddc-i2c-rs/master.svg?style=flat-square
[travis]: https://travis-ci.org/arcnmx/ddc-i2c-rs
[release-badge]: https://img.shields.io/crates/v/ddc-i2c.svg?style=flat-square
[cargo]: https://crates.io/crates/ddc-i2c
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: http://arcnmx.github.io/ddc-i2c-rs/ddc_i2c/
[license-badge]: https://img.shields.io/badge/license-MIT-ff69b4.svg?style=flat-square
[license]: https://github.com/arcnmx/ddc-i2c-rs/blob/master/COPYING
