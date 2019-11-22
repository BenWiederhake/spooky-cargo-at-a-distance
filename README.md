# Spooky cargo at a distance

This is the code in question:

```Rust
use rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;

fn main() {
    Xoshiro256Plus::from_entropy();
}
```

## Implicit effect of `Cargo.toml` dependencies

This seems to be using `SeedableRng` from `rand_core`, which does not define `from_entropy()`.
However, with this in `Cargo.toml` it just works:

```
[dependencies]
rand_xoshiro = "0.4.0"
rand_core = "0.5.1"
rand = "0.7.2"  # Uncomment this line and everything breaks.  Huh?!
```

leads to:

```
$ cargo build
    Updating crates.io index
    Finished dev [unoptimized + debuginfo] target(s) in 0.53s
```

However, just removing a dependency there, not even touching `main.rs`, then everything breaks:

```
[dependencies]
rand_xoshiro = "0.4.0"
rand_core = "0.5.1"
```

leads to:

```
$ cargo build
   Compiling rng-trouble v0.1.0 (/scratch/rng-trouble)
warning: unused import: `rand_core::SeedableRng`
 --> src/main.rs:1:5
  |
1 | use rand_core::SeedableRng;
  |     ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0599]: no function or associated item named `from_entropy` found for type `rand_xoshiro::xoshiro256plus::Xoshiro256Plus` in the current scope
 --> src/main.rs:5:21
  |
5 |     Xoshiro256Plus::from_entropy();
  |                     ^^^^^^^^^^^^ function or associated item not found in `rand_xoshiro::xoshiro256plus::Xoshiro256Plus`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: Could not compile `rng-trouble`.

To learn more, run the command again with --verbose.
```

## Under the Hood

The crate `rand_core` defines [`SeedableRng`](https://docs.rs/rand_core/0.5.1/rand_core/trait.SeedableRng.html), which does *not* define `from_entropy()`.
Meanwhile, the crate `rand` defines [`SeedableRng`](https://docs.rs/rand/0.7.2/rand/trait.SeedableRng.html#method.from_entropy), which *does* define `from_entropy()`.

However, `main.rs` clearly says `use rand_core::SeedableRng;`, so I'm not sure how that works.

## Not a versions-issue

Note that this has nothing to do with versions, as in both cases the same versions (or nothing) are used.

Excerpt from `Cargo.lock` with `rand`:

```
"checksum rand 0.7.2 (registry+https://github.com/rust-lang/crates.io-index)" = "3ae1b169243eaf61759b8475a998f0a385e42042370f3a7dbaf35246eacc8412"
"checksum rand_chacha 0.2.1 (registry+https://github.com/rust-lang/crates.io-index)" = "03a2a90da8c7523f554344f921aa97283eadf6ac484a6d2a7d0212fa7f8d6853"
"checksum rand_core 0.5.1 (registry+https://github.com/rust-lang/crates.io-index)" = "90bde5296fc891b0cef12a6d03ddccc162ce7b2aff54160af9338f8d40df6d19"
"checksum rand_hc 0.2.0 (registry+https://github.com/rust-lang/crates.io-index)" = "ca3129af7b92a17112d59ad498c6f81eaf463253766b90396d39ea7a39d6613c"
"checksum rand_xoshiro 0.4.0 (registry+https://github.com/rust-lang/crates.io-index)" = "a9fcdd2e881d02f1d9390ae47ad8e5696a9e4be7b547a1da2afbc61973217004"
```

Excerpt from `Cargo.lock` without `rand`:

```
"checksum rand_core 0.5.1 (registry+https://github.com/rust-lang/crates.io-index)" = "90bde5296fc891b0cef12a6d03ddccc162ce7b2aff54160af9338f8d40df6d19"
"checksum rand_xoshiro 0.4.0 (registry+https://github.com/rust-lang/crates.io-index)" = "a9fcdd2e881d02f1d9390ae47ad8e5696a9e4be7b547a1da2afbc61973217004"
```

Yes, I deleted `Cargo.lock` between the runs.
