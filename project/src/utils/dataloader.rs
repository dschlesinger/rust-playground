use csv::ReaderBuilder;
use ndarray::{Array1, Array2, s};
use std::fs::File;
use std::error::Error;
use rand::thread_rng;
use rand::prelude::SliceRandom;

pub struct Iris {
    pub x_train: Array2<f32>,
    pub x_test: Array2<f32>,
    pub y_train: Vec<String>,
    pub y_test: Vec<String>,
}

impl Iris {
    pub fn load_train(&self, path: &str) -> Result<(Array2<f32>, Vec<String>), Box<dyn Error>> {
        self.load_data(path, 0, 100)
    }

    pub fn load_test(&self, path: &str) -> Result<(Array2<f32>, Vec<String>), Box<dyn Error>> {
        self.load_data(path, 100, 150)
    }

    pub fn load_data(&self, path: &str, start_slice: usize, stop_slice: usize) -> Result<(Array2<f32>, Vec<String>), Box<dyn Error>> {
        let mut features: Vec<f32> = Vec::new();
        let mut labels: Vec<String> = Vec::new();
        let mut num_columns = 0;
        let mut row_count = 0;
    
        let file = File::open(path)?;
        let mut reader = ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .flexible(true)
            .from_reader(file);
    
        let mut first_line = true;
    
        for result in reader.records() {
            if first_line {
                first_line = false;
                continue;
            }
    
            let record = result?;
    
            if num_columns == 0 {
                num_columns = record.len() - 1;
            }
    
            labels.push(record.get(num_columns).ok_or("Missing class label")?.to_string());
    
            for i in 0..num_columns {
                let value = record.get(i).ok_or("Missing feature value")?.parse::<f32>()?;
                features.push(value);
            }
    
            row_count += 1;
        }
    
        let x_array2 = Array2::from_shape_vec((row_count, num_columns), features)?;
    
        let mut combined: Vec<_> = x_array2
            .outer_iter()
            .zip(labels.into_iter())
            .map(|(row, label)| (row.to_owned(), label))
            .collect();
    
        let mut rng = thread_rng();
        combined.shuffle(&mut rng);
    
        let (rows, shuffled_labels): (Vec<_>, Vec<_>) = combined.into_iter().unzip();
    
        // Stack rows back into Array2
        let x_stacked = ndarray::stack(ndarray::Axis(0), &rows.iter().map(|r| r.view()).collect::<Vec<_>>())?;
    
        // Slice the final Array2 and labels
        let sliced_x = x_stacked.slice(s![start_slice..stop_slice, ..]).to_owned();
        let sliced_y = shuffled_labels[start_slice..stop_slice].to_vec();
    
        Ok((sliced_x, sliced_y))
    }
    
    
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let (x_train, y_train) = Self {
            x_train: Array2::zeros((0, 0)),
            x_test: Array2::zeros((0, 0)),
            y_train: Vec::new(),
            y_test: Vec::new(),
        }.load_train(path)?;
        
        let (x_test, y_test) = Self {
            x_train: Array2::zeros((0, 0)),
            x_test: Array2::zeros((0, 0)),
            y_train: Vec::new(),
            y_test: Vec::new(),
        }.load_test(path)?;
        
        Ok(Self {
            x_train,
            x_test,
            y_train,
            y_test,
        })
    }
}