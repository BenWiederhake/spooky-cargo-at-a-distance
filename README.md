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
