use polars::prelude::*;

fn main() {
    let df: DataFrame = df!(
        "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
        "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
        "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
    )
    .unwrap();
    let weight_differences = diff(
        df["weight"].as_series().unwrap(),
        1,
        polars::series::ops::NullBehavior::Drop,
    )
    .unwrap();
    println!("{weight_differences}",);
}
