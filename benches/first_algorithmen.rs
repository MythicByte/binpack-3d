use std::hint::black_box;

use algorithmen_test3::{
    algorithmen::Algorithmen3DBinPackaging,
    bin::Bin,
    first_algorithmen::AlgorithmenFirst,
    items::Item,
    vector::Vector3,
};
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
    let mut _rng = ChaCha20Rng::from_seed(seed);
    let mut group = c.benchmark_group("First Algorithmen");
    let bin = Bin::new(Vector3::new(1000, 1000, 1000), 100000, 0);
    let items: Vec<Item> = (0..100)
        .map(|_| Item::new(Vector3::new(10, 10, 10), 10, 1))
        .collect();
    let algorithmen_in = match AlgorithmenFirst::create_algorithmen(items, bin).unwrap() {
        algorithmen_test3::algorithmen::AlgorithmenCreation::WorkedButToMuchItems {
            algorithmen,
            items_to_much,
        } => algorithmen,
        algorithmen_test3::algorithmen::AlgorithmenCreation::NoProblems(x) => x,
    };
    group.bench_function("EqualCheck", |b| {
        b.iter(|| {
            black_box(algorithmen_in.clone().calculate().unwrap());
        })
    });
}
criterion_group!(benches, check_first_algorithmen);
criterion_main!(benches);
