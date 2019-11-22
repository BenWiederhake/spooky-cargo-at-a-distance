use rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;

fn main() {
    Xoshiro256Plus::from_entropy();
}
