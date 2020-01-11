# Rust wrapper around the FST API of Gtkwave

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.txt)
[![Crates.io Version](https://img.shields.io/crates/v/fst-sys.svg)](https://crates.io/crates/fst-sys)
[![Doc.rs Documentation](https://docs.rs/fst-sys/badge.svg)](https://docs.rs/fst-sys/)

This crate exposes raw bindings to GTKWave FST API in Rust. 
The C sources included in this repository (see [fstapi](./fstapi)) are copied from [Gtkwave source repository](https://sourceforge.net/projects/gtkwave/).

## Update bindings

The bindings are automatically generated into `src/bindings.rs` using [bindgen](https://github.com/rust-lang/rust-bindgen.git).
To regenerate them, just run: 
```console
$ ./regen.sh
```

## Licensing

The project is licensed under a MIT license (see [LICENSE.txt](./LICENSE.txt)). 
However, it uses the following subprojects:

* fstapi, under MIT (see [LICENSE-fstapi.txt](./LICENSE-fstapi.txt))
* fastlz, under MIT (see [LICENSE-fastlz.txt](./LICENSE-fastlz.txt))
* lz4, under BSD-2 (see [LICENSE-lz4.txt](./LICENSE-lz4.txt))