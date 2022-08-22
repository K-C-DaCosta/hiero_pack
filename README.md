# Hiero Pack
This is a simple parser and packing tool for libgdx's  <a href="https://github.com/libgdx/libgdx/wiki/Hiero">Hiero</a>

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
