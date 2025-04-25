use ndarray::{ Array, Array2};
use std::f32::consts::E;

pub fn ReLU(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| v.max(0.0))
}

pub fn dReLU(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| if v > 0.0 { 1.0 } else { 0.0 })
}

pub fn Sigmoid(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| { 1.0 / (1.0 + E.powf(-v)) })
}

pub fn dSigmoid(x: &Array2<f32>) -> Array2<f32> {
    let sig = Sigmoid(x);
    sig.mapv(|v| v * (1.0 - v))
}

pub fn Softmax(x: &Array2<f32>) -> Array2<f32> {
    let max = x.fold(std::f32::NEG_INFINITY, |a, &b| a.max(b));
    let exps = x.mapv(|v| (v - max).exp());
    let sum = exps.sum();
    exps / sum
}


// Not an activation but whatever

pub fn CCE(logits: &Array2<f32>, target: &Array2<f32>) -> f32 {

    let mut target_index = 11;

    for (idx, val) in target.iter().enumerate() {
        if *val == 1.0 {
            target_index = idx as usize;
            break;
        }
    }

    let correct_logit = logits[[target_index, 0]];
    let log_sum_exp = logits.mapv(|z| z.exp()).sum().ln();
    -correct_logit + log_sum_exp
}

pub fn dCCE(logits: &Array2<f32>, target: &Array2<f32>) -> Array2<f32> {

    Softmax(&logits) - target

}

pub fn Accuracy(logits: &Vec<Array2<f32>>, target: &Vec<Array2<f32>>) -> f32 {

    let mut total: i32 = 0;

    for (l, t) in logits.iter().zip(target.iter()) {

        // Find argmax

        let mut argmax: i32 = -1;
        let mut valmax: f32 = -1.0;

        for (idx, li) in l.iter().enumerate() {

            if *li > valmax {

                valmax = *li;
                argmax = idx as i32;

            }

        }

        // if argmax is 1.0 then +1

        if t[[argmax as usize, 0]] == 1.0 {

            total += 1;

        }

    }

    return (total as f32) / (logits.len() as f32);

}