use ndarray::{Array1, ArrayView1, Array2, Axis};
use super::math::{euc};

pub struct Prediction {
    IrisSetosa: i32,
    IrisVersicolor: i32,
    IrisVirginica: i32,
    total: i32,
}

impl Prediction {
    pub fn new(labels: Vec<String>) -> Self {
        let mut IrisSetosa = 0;
        let mut IrisVersicolor = 0;
        let mut IrisVirginica = 0; 

        for l in labels.iter() {
            match l.as_str() { 
                "Iris-setosa" => IrisSetosa += 1,
                "Iris-versicolor" => IrisVersicolor += 1,
                "Iris-virginica" => IrisVirginica += 1,
                _ => panic!("{} is not a flower", l)
            }
        }

        let total = IrisSetosa + IrisVersicolor + IrisVirginica;

        Prediction { IrisSetosa, IrisVersicolor, IrisVirginica, total  }
    }

    pub fn sample(self: &Self) -> String {

        if self.IrisSetosa > self.IrisVersicolor && self.IrisSetosa > self.IrisVirginica {
            return "Iris-setosa".to_string();
        }

        else if self.IrisVersicolor > self.IrisVirginica {
            return "Iris-versicolor".to_string();
        }

        else {
            return "Iris-virginica".to_string();
        }

    }

    pub fn print_dist(self: &Self) -> () {

        print!("Iris-Setosa: {:^10} | ", self.IrisSetosa as f32 / self.total as f32);
        print!("Iris-Versicolor: {:^10} | ", self.IrisVersicolor as f32 / self.total as f32);
        print!("Iris-Virginica: {:^10}", self.IrisVirginica as f32 / self.total as f32);
        println!("");

    }

}

pub struct KNN {
    data: Array2<f32>,
    labels: Vec<String>,
    n_neighbors: i32,
}

impl KNN {

    pub fn new(data: Array2<f32>, labels: Vec<String>, n_neighbors: i32) -> Self {

        if (data.len() as i32) < n_neighbors {
            panic!("Data is larger then n_neighbors");
        }

        KNN { data, labels, n_neighbors }
    }

    fn set_n_neighbors(self: &mut Self, n_neighbors: i32) -> () {
        self.n_neighbors = n_neighbors;
    }

    pub fn predict(self: &Self, datapoint: ArrayView1<f32>) -> String {

        let mut euc_min: Vec<f32> = vec![1000.0; self.n_neighbors as usize];
        let mut pos: Vec<usize> = vec![0; self.n_neighbors as usize];

        // Find top n_neighbors positons
        for (r_num, row) in self.data.axis_iter(Axis(0)).enumerate() {

            let euc_val = euc(row, datapoint.clone());

            for (idx, min_val) in euc_min.iter().enumerate() {

                if (*min_val > euc_val) {

                    // Push into val position
                    euc_min.insert(idx as usize, euc_val);

                    // Push into index position
                    pos.insert(idx as usize, r_num as usize);

                    // Clean ends
                    euc_min.pop();
                    pos.pop();
                    break;

                }

            }

        }

        // Get labels of those positons

        let mut labels: Vec<String> = vec![];

        for i in pos.iter() {
            labels.push(self.labels[*i].clone());
        }

        let P = Prediction::new(labels);

        return P.sample();

    }

}