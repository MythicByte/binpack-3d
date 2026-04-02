use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use rand::{
    prelude::*,
    rngs::ChaCha20Rng,
};
fn check_first_algorithmen(c: &mut Criterion) {
    let seed = [42u8; 32];
    let mut rng = ChaCha20Rng::from_seed(seed);
    let group = c.benchmark_group("First Algorithmen");
}
criterion_group!(benches, check_first_algorithmen);
criterion_main!(benches);
