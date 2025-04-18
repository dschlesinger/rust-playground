use ndarray::{s, Array, Array2};
use std::f64::consts::E;

pub fn ReLU(x: &Array2<f64>) -> Array2<f64> {
    x.mapv(|v| v.max(0.0))
}

pub fn dReLU(x: &Array2<f64>) -> Array2<f64> {
    x.mapv(|v| if v > 0.0 { 1.0 } else { 0.0 })
}

pub fn Sigmoid(x: &Array2<f64>) -> Array2<f64> {
    x.mapv(|v| { 1.0 / (1.0 + E.powf(-v)) })
}

pub fn dSigmoid(x: &Array2<f64>) -> Array2<f64> {
    let sig = Sigmoid(x);
    sig.mapv(|s| s * (1.0 - s))
}