use algorithmen_test3::{
    algorithmen::Algorithmen3DBinPackaging,
    bin::Bin,
    first_algorithmen::AlgorithmenFirst,
    items::Item,
};
use nalgebra::Vector3;

#[test]
fn algorithmenfirst_check_basic() {
    let bin = Bin::new(Vector3::new(1000, 1000, 1000), 100000, 0);
    let items: Vec<Item> = (0..100)
        .map(|_| Item::new(Vector3::new(10, 10, 10), 10, 1))
        .collect();
    let algorithmen_in = AlgorithmenFirst::create_algorithmen(items, bin).unwrap();
    assert_eq!(algorithmen_in.calculate().is_ok(), true);
}
