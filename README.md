# `easyrand`

[![Build Status](https://travis-ci.org/dbr/easyrand-rs.svg?branch=master)](https://travis-ci.org/dbr/easyrand-rs) [![Documentation](https://docs.rs/easyrand/badge.svg)](https://docs.rs/easyrand)

Simpler to use random number library for the Rust language.

The library wraps the [`rand`][rand] crate, exposing a smaller API somewhat inspired by the Python [`random`][random] module. Similar in concept to [`reqwest`][reqwest] being a wrapper for the more complex [`hyper`][hyper].


# Usage

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


[rand]: https://crates.io/crates/rand
[reqwest]: https://crates.io/crates/reqwest
[hyper]: https://crates.io/crates/hyper
[random]: https://docs.python.org/3/library/random.html