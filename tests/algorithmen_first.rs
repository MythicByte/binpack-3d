use algorithmen_test3::{
    algorithmen::Algorithmen3DBinPackaging,
    bin::Bin,
    first_algorithmen::AlgorithmenFirst,
    items::Item,
    vector::Vector3,
};
use rand::RngExt;

// Helper to generate random valid items and bin
fn generate_valid_test_case() -> (Bin, Vec<Item>) {
    let mut rng = rand::rng();

    // Generate bin with random dimensions (10-100 units) and capacity
    let bin_size = Vector3::new(
        rng.random_range(10..100),
        rng.random_range(10..100),
        rng.random_range(10..100),
    );
    let bin_weight_capacity = rng.random_range(1000..10000);
    let bin = Bin::new(bin_size, bin_weight_capacity, 0);

    // Generate 5-20 items with dimensions < bin and total volume/weight < bin
    let item_count = rng.random_range(5..20);
    let mut items = Vec::with_capacity(item_count);
    let mut total_volume = 0u32;
    let mut total_weight = 0u32;

    for _ in 0..item_count {
        // Ensure item dimensions are smaller than bin in all axes
        let item_size = Vector3::new(
            rng.random_range(1..bin_size.x),
            rng.random_range(1..bin_size.y),
            rng.random_range(1..bin_size.z),
        );
        let item_weight = rng.random_range(1..100);

        let item_volume = item_size.x * item_size.y * item_size.z;

        // Ensure we don't exceed 90% of bin capacity for buffer
        if ((total_volume + item_volume) as f32)
            < ((bin_size.x * bin_size.y * bin_size.z) as f32) * 0.9
        {
            items.push(Item::new(item_size, item_weight, 0));
            total_volume += item_volume;
            total_weight += item_weight;
        }
    }

    (bin, items)
}

#[test]
fn algorithmenfirst_random_valid() {
    let (bin, items) = generate_valid_test_case();
    let algorithmen_in = match AlgorithmenFirst::create_algorithmen(items, bin).unwrap() {
        algorithmen_test3::algorithmen::AlgorithmenCreation::NoProblems(x) => x,
        _ => panic!("Expected valid test case"),
    };
    let result = algorithmen_in.calculate();
    dbg!(&result);
    assert!(result.is_ok());
}

#[test]
fn algorithmenfirst_random_invalid() {
    let mut rng = rand::rng();

    // Generate bin with random dimensions
    let bin_size = Vector3::new(
        rng.random_range(10..100),
        rng.random_range(10..100),
        rng.random_range(10..100),
    );
    let bin_weight_capacity = rng.random_range(1000..10000);
    let bin = Bin::new(bin_size, bin_weight_capacity, 0);

    // Generate items that definitely exceed bin capacity
    let item = Item::new(
        bin_size + Vector3::new(1, 1, 1), // Slightly larger than bin
        bin_weight_capacity + 1,
        0,
    );
    let items = vec![item];

    let result = AlgorithmenFirst::create_algorithmen(items, bin);
    dbg!(&result);
    assert!(
        result.unwrap_err() == algorithmen_test3::algorithmen::AlgorithmenError::NotEnoughSpace
    );
}

#[test]
fn algorithmenfirst_random_edge_cases() {
    let mut rng = rand::rng();

    // Test exact fit
    let bin_size = Vector3::new(10, 10, 10);
    let bin = Bin::new(bin_size, 1000, 0);
    let item = Item::new(bin_size, 1000, 0);
    let result = match AlgorithmenFirst::create_algorithmen(vec![item], bin).unwrap() {
        algorithmen_test3::algorithmen::AlgorithmenCreation::WorkedButToMuchItems {
            algorithmen,
            items_to_much,
        } => algorithmen,
        algorithmen_test3::algorithmen::AlgorithmenCreation::NoProblems(x) => x,
    };
    assert!(result.calculate().is_ok());

    // Test single item that fits
    let bin = Bin::new(Vector3::new(20, 20, 20), 8000, 0);
    let item = Item::new(Vector3::new(10, 10, 10), 100, 0);
    let result = match AlgorithmenFirst::create_algorithmen(vec![item], bin).unwrap() {
        algorithmen_test3::algorithmen::AlgorithmenCreation::WorkedButToMuchItems {
            algorithmen,
            items_to_much,
        } => algorithmen,
        algorithmen_test3::algorithmen::AlgorithmenCreation::NoProblems(x) => x,
    };
    assert!(result.calculate().is_ok());
}
