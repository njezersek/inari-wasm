# 🦊 inari wasm 🕸

**inari_wasm** is a striped down version of original [inari](https://github.com/unageek/inari) library which is a Rust implementation of [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic).

Original library uses [`gmp-mpfr-sys`](https://crates.io/crates/gmp-mpfr-sys) that allows it to specify different floating point rounding policies for calculating lower and upper bounds of an interval. Unfortunaly web assembly does not suport instructions necessary to change the rounding policy (https://github.com/WebAssembly/design/issues/1384). To use this library in web assembly I had to remove all functions from `gmp` and replace them with their normal counter parts.

Original library uses SIMD instructions. These are supported in web assembly but I don't know how to use them so they were also replaced.

I modified the library by copying the functions one by one to a new project and modifing them. I included the `Interval` struct and all of its implementations. For now I haven't included `DecInterval`.

> This version is less accurate and slower than original if you don't need to compile for wasm, please use the [original library](https://github.com/unageek/inari). 

## Example
```rust
let x = const_interval!(0.0, 2.0);
let y = x.sin() + const_interval!(1.0);
println!("{}", y); // [1, 2]
```

## References

- Inari - A Rust implementation of interval arithmetic (IEEE 1788). https://github.com/unageek/inari
- IEEE Std 1788-2015 - IEEE Standard for Interval Arithmetic. https://doi.org/10.1109/IEEESTD.2015.7140721
- IEEE Std 1788.1-2017 - IEEE Standard for Interval Arithmetic (Simplified). https://doi.org/10.1109/IEEESTD.2018.8277144
