use ndarray::{ArrayView1, Array1, Array2};

pub fn euc(a: ArrayView1<f32>, b: ArrayView1<f32>) -> f32 {
    a.iter()
     .zip(b.iter())
     .map(|(&x, &y)| (x - y).powi(2))
     .sum::<f32>()
     .sqrt()
}