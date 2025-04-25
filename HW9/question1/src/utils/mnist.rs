use ndarray::{Array3, Array2, Array};
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

pub struct MNIST;

impl MNIST {
    pub fn load_train(&self) -> Result<(Vec<Array2<f32>>, Vec<Array2<f32>>), Box<dyn Error>> {
        self.load_data("mnist/mnist_train.csv")
    }

    pub fn load_test(&self) -> Result<(Vec<Array2<f32>>, Vec<Array2<f32>>), Box<dyn Error>> {
        self.load_data("mnist/mnist_test.csv")
    }

    pub fn load_data(&self, path: &str) -> Result<(Vec<Array2<f32>>, Vec<Array2<f32>>), Box<dyn Error>> {
        let mut x = Vec::with_capacity(50000);
        let mut y = Vec::with_capacity(50000);

        let file = File::open(path)?;
        let mut reader = ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .flexible(true)
            .from_reader(file);

        for line in reader.records() {
            let record = line?;
            let mut parts = record.iter();

            let label_str = parts.next().ok_or("Missing label")?;
            let label: usize = label_str.parse()?;

            let mut yi = Array::zeros((10, 1));
            yi[[label, 0]] = 1.0;

            let image_data: Vec<f32> = parts.map(|v| v.parse::<f32>()).collect::<Result<_, _>>()?;
            let mut xi = Array2::from_shape_vec((784, 1), image_data)?;

            // Normalize
            xi = xi.mapv(|v| v / 255.0);

            x.push(xi);
            y.push(yi);
        }

        Ok((x, y))
    }
}
