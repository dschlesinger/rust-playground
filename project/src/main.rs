mod utils;

use ndarray::{Array1, Array2, Axis};
use utils::knn::{KNN, Prediction};
use utils::dataloader::{Iris};

fn main() {
    let data: Iris = Iris::new("data/Iris.csv").expect("Failed to load csv");

    let model = KNN::new(data.x_train.clone(), data.y_train.clone(), 32);

    let mut total: i32 = 0;

    for (idx, r) in data.x_test.axis_iter(Axis(0)).enumerate() {
        if model.predict(r.clone()) == data.y_test[idx] {
            total += 1;
        }
    }

    println!("KNN is {}% accurate", 100.0 * (total as f32) / data.y_test.len() as f32);
}
