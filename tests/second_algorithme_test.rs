use algorithmen_test3::{
    algorithmen::Algorithmen3DBinPackaging,
    bin::Bin,
    items::Item,
    second_algorithmen::SecondAlgorithmen,
    vector::Vector3,
};
use hashbrown::HashSet;

#[test]
fn bin_fix_v2() {
    let bin = Bin::new(1, Vector3::new(1000, 1000, 1000), 100000, 0);
    let item = Item::new(1, Vector3::new(10, 10, 10), 10, 1);
    let mut list = Vec::with_capacity(1000);
    for _ in 0..1000 {
        list.push(item.clone());
    }
    let algorithmen = SecondAlgorithmen::create_algorithmen(list, bin).unwrap();
    let result = algorithmen.calculate().unwrap();
    assert_eq!(1000, result.items.len());
    assert_eq!(0, result.removed_items.len());
    let mut hash_check: HashSet<Vector3<u32>> = HashSet::with_capacity(result.items.len());
    for i in result.items {
        if !hash_check.insert(i.position) {
            panic!("Same corner was used");
        }
    }
}
#[test]
fn bin_variable_v3() {
    let x = || return rand::random_range(0u32..100u32);
    let bin = Bin::new(1, Vector3::new(10000, 10000, 10000), 100000, 0);
    let item = Item::new(1, Vector3::new(x(), x(), x()), 10, 1);
    let mut list = Vec::with_capacity(1000);
    for _ in 0..1000 {
        list.push(item.clone());
    }
    let algorithmen = SecondAlgorithmen::create_algorithmen(list, bin).unwrap();
    let result = algorithmen.calculate().unwrap();
    assert_eq!(1000, result.items.len());
    assert_eq!(0, result.removed_items.len());
    let mut hash_check: HashSet<Vector3<u32>> = HashSet::with_capacity(result.items.len());
    for i in result.items {
        if !hash_check.insert(i.position) {
            panic!("Same corner was used");
        }
    }
}
