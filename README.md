# `easyrand`

[![Build Status][build_img]][build_link] [![Crates.io version][crate_img]][crate_link] [![Documentation][doc_img]][doc_link]

Simple to use random number library for the Rust language.

The library wraps the [`rand`][rand] crate, exposing a smaller API somewhat inspired by the Python [`random`][random] module. Similar in concept to [`reqwest`][reqwest] being a wrapper for the more complex [`hyper`][hyper].

[rand]: https://crates.io/crates/rand
[reqwest]: https://crates.io/crates/reqwest
[hyper]: https://crates.io/crates/hyper
[random]: https://docs.python.org/3/library/random.html

[build_img]: https://github.com/dbr/easyrand-rs/actions/workflows/test.yml/badge.svg
[build_link]: https://github.com/dbr/easyrand-rs/actions/workflows/test.yml
[doc_img]: https://docs.rs/easyrand/badge.svg
[doc_link]: https://docs.rs/easyrand
[crate_img]: https://img.shields.io/crates/v/easyrand
[crate_link]: https://crates.io/crates/easyrand

# Usage

Add to `Cargo.toml`:

    [dependencies]
    easyrand="0.4"

Then to use:

    extern crate easyrand;

    // Generate a random f64 between 0 and 1
    let r = easyrand::random();

    // Or for a given range
    let r = easyrand::randrange(0.0, 10.0);

    // Generate a random integer
    let r = easyrand::randint(0, 99);

    // Shuffle a vector
    let mut inputs = vec!["a", "b", "c", "d"];
    easyrand::shuffle(&mut inputs);


# Project scope

This library is intended to be a small subset of the `rand` library, exposing only the simplest possible useful subset of the library. If you need anything more specific, use the underlying `rand` library.
