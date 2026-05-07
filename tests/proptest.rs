use binpack_3d::{
    algorithmen::Algorithmen3DBinPackaging,
    bin::Bin,
    items::Item,
    second_algorithmen::SecondAlgorithmen,
    vector::Vector3,
};
use proptest::proptest;
proptest! {
    #[test]
    fn random_input_dpes_not_fail_first_algorithmen(x in 0u32..100) {
        let bin = Bin::new(1,Vector3::new(1000,1000,1000),100000,0);
        let item = Item::new(1,Vector3::new(10,10,10),10,1);
        let mut item_list:Vec<Item> = Vec::with_capacity(x as usize);
        for _ in 0..x {
            item_list.push(item.clone());
        }
        let mut algorithmus = SecondAlgorithmen::create_algorithmen(item_list, bin).unwrap();
        let result = algorithmus.calculate().unwrap();
        // dbg!(&result);
    }
}
