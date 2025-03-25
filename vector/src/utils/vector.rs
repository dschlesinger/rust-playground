use num_traits::Num;

pub struct Matrix<T: Num + Copy> {
    data: Vec<T>,
    shape: Vec<i32>,
    dim: i8,
}

impl<T: Num + Copy> Matrix<T> {

    const MAX_DIM: i8 = i8::MAX;

    pub fn new(data: Vec<T>) -> Self {

        // Get shape and check for uniformity

        while !data[0].is::<T>() {

        }

        Matrix { data }
    }

    pub fn dim(&self) -> i8 {

        for 

    }

}