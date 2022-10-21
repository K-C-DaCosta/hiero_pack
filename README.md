# Hiero Pack
[![Build status](https://github.com/K-C-DaCosta/hiero_pack/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/K-C-DaCosta/hiero_pack/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/hiero_pack)](https://crates.io/crates/hiero_pack)
[![Documentation](https://docs.rs/hiero_pack/badge.svg)](https://docs.rs/hiero_pack)


This is a simple parser and packing tool for libgdx's  <a href="https://libgdx.com/wiki/tools/hiero">Hiero</a>

# How to compile 
```
git clone [this repo]
cargo build --release
```

# How to pack 
After you've used hiero to generate your fonts we just do a:
```
cargo run --release -- --font foo.a --pages page1.png page2.png 
```
And wahlah the output should be a packed, slightly more compact, atlus in binary.
