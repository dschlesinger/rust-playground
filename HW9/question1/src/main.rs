use ndarray::{s, Array, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use std::{error::Error, fs::File};

mod utils;
use utils::activations::{ReLU, dReLU, Softmax, CCE, dCCE, Accuracy};
use utils::mnist::MNIST;

struct NeuralNetwork {
    input_size: usize,
    layer1_size: usize,
    layer2_size: usize,
    output_size: usize,
    learning_rate: f32,
    weights_input_to_layer1: Array2<f32>,
    weights_layer1_to_layer2: Array2<f32>,
    weights_layer2_to_output: Array2<f32>,
}

#[derive(Debug)]
struct LayerLogits {
    l1: Array2<f32>,
    l1r: Array2<f32>,
    l2: Array2<f32>,
    l2r: Array2<f32>,
    l3: Array2<f32>,
    l3s: Array2<f32>,
}

impl NeuralNetwork {
    fn new(input_size: usize, layer1_size: usize, layer2_size: usize, output_size: usize, learning_rate: f32) -> Self {
        let weights_input_to_layer1 = Array::random((layer1_size, input_size), Uniform::new(-0.1, 0.1));
        let weights_layer1_to_layer2 = Array::random((layer2_size, layer1_size), Uniform::new(-0.1, 0.1));
        let weights_layer2_to_output = Array::random((output_size, layer2_size), Uniform::new(-0.1, 0.1));

        NeuralNetwork {
            input_size,
            layer1_size,
            layer2_size,
            output_size,
            learning_rate,
            weights_input_to_layer1,
            weights_layer1_to_layer2,
            weights_layer2_to_output,
        }
    }

    fn forward(&self, input: &Array2<f32>) -> LayerLogits {
        let layer1_output = self.weights_input_to_layer1.dot(input);

        let layer1_relu = ReLU(&layer1_output);

        let layer2_output = self.weights_layer1_to_layer2.dot(&layer1_relu);

        let layer2_relu = ReLU(&layer2_output);

        let layer3_output = self.weights_layer2_to_output.dot(&layer2_relu);

        let final_output = Softmax(&layer3_output);

        LayerLogits {
            l1: layer1_output,
            l1r: layer1_relu,
            l2: layer2_output,
            l2r: layer2_relu,
            l3: layer3_output,
            l3s: final_output,
        }
    }

    fn backward(&mut self, LL: &LayerLogits, input: &Array2<f32>, targets: &Array2<f32>) {

        let dz3 = dCCE(&LL.l3, targets);
        let dw3 = dz3.dot(&LL.l2r.t());

        let dz2 = self.weights_layer2_to_output.t().dot(&dz3) * dReLU(&LL.l2);
        let dw2 = dz2.dot(&LL.l1r.t());

        let dz1 = self.weights_layer1_to_layer2.t().dot(&dz2) * dReLU(&LL.l1);
        let dw1 = dz1.dot(&input.t());

        self.weights_layer2_to_output -= &(dw3 * self.learning_rate);
        self.weights_layer1_to_layer2 -= &(dw2 * self.learning_rate);
        self.weights_input_to_layer1 -= &(dw1 * self.learning_rate);
    }
}

fn main() {
    let mut network = NeuralNetwork::new(784, 128, 64, 10, 0.01);
    let (x_train, y_train) = MNIST.load_train().unwrap();
    let (x_test, y_test) = MNIST.load_test().unwrap();

    for epoch in 0..3 {

        println!("Starting Epoch {}", epoch);

        for example in 0..x_train.len() {
            let f = network.forward(&x_train[example]);
            network.backward(&f, &x_train[example], &y_train[example]);
        }
    }

    let mut test_results: Vec<Array2<f32>> = Vec::with_capacity(10000);

    for example in 0..x_test.len() {
        let f = network.forward(&x_test[example]);

        test_results.push(f.l3s);
    }

    println!("Accuracy {}", Accuracy(&test_results, &y_test));
}