# Rust wrapper around the FST API of Gtkwave

This is a work in progress attempt to wrap the FST API in Rust. The C library 
(see [fstapi](./fstapi)) has been imported from [Gtkwave source repository](https://sourceforge.net/projects/gtkwave/), 

## Usage

See [examples/main.rs](examples/main.rs) and run it with:

```console
$ cargo run --example main <path to some FST file>
```
