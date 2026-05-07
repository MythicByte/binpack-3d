use std::hint::black_box;

use binpack_3d::{
    algorithmen::Algorithmen3DBinPackaging,
    bin::Bin,
    items::Item,
    second_algorithmen::SecondAlgorithmen,
    vector::Vector3,
};
use criterion::{
    BatchSize,
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
    let mut group = c.benchmark_group("Second Algorithmen");
    group.sample_size(10);
    let bin = Bin::new(1, Vector3::new(10000, 10000, 10000), 10000000, 0);
    let items: Vec<Item> = (0..10000)
        .map(|x| Item::new(x, Vector3::new(10, 10, 10), 10, 1))
        .collect();
    let algorithmen_in = SecondAlgorithmen::create_algorithmen(items, bin).unwrap();
    // group.bench_function("Second Algorithmen 10000 rotations", |b| {
    //     b.iter(|| {
    //         black_box(algorithmen_in.clone().calculate().unwrap());
    //     })
    // });
    group.bench_function("Second Algorithmen 10000 items", |b| {
        b.iter_batched(
            // 1. Setup: This happens outside the timer
            || algorithmen_in.clone(),
            // 2. The Routine: Only this part is timed
            |instance| black_box(instance.calculate().unwrap()),
            // 3. Batch Size: Since 10k items is a lot of memory,
            // LargeInput ensures we don't run out of RAM during setup.
            BatchSize::LargeInput,
        );
    });
}
criterion_group!(benches, check_first_algorithmen);
criterion_main!(benches);
