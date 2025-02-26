trait Vector2D {
    fn new(data: [[i32; 5]; 5]) -> Self;

    // Flatten method that returns a flattened vector of i32
    fn flatten(&self) -> [i32; 25];

    fn sum(&self) -> i32;

    fn dot(&self, other: &Self) -> i32;

    fn transpose(&self) -> Self;

    fn matmul(&self, other: &Self) -> Self;
}

// Implementing the trait for a struct
struct Matrix2D {
    data: [[i32; 5]; 5], // Store a fixed-size 5x5 array
}

impl Vector2D for Matrix2D {
    fn new(data: [[i32; 5]; 5]) -> Self {
        Matrix2D { data }
    }

    fn matmul(&self, other: &Self) -> Self {
        let mut result = [[0; 5]; 5];

        for i in 0..5 {
            for j in 0..5 {
                for k in 0..5 {
                    result[i][j] = self.data[i][k] * other.data[k][j];
                }
            }
        }

        return Matrix2D::new(result);
    
    }

    fn transpose(&self) -> Self {
        let mut transposed = [[0; 5]; 5];

        for i in 0..5 {
            for j in 0..5 {
                transposed[i][j] = self.data[j][i];
            }
        }

        return Matrix2D::new(transposed);
    }

    fn dot(&self, other: &Self) -> i32 {
        let mut total = 0;

        for i in 0..5 {
            for j in 0..5 {
                total += self.data[i][j] * other.data[i][j];
            }
        }

        return total;
    }

    fn flatten(&self) -> [i32; 25] {

        let mut flattened = [0; 25];
        let mut index: u8 = 0;

        for row in self.data.iter() {
            for &element in row.iter() {
                flattened[index as usize] = element;
                index += 1;
            }
        }

        return flattened;
    }

    fn sum(&self) -> i32 {

        let mut total = 0;
        
        for row in self.data.iter() {
            for &element in row.iter() {
                total += element;
            }
        }

        return total;
    }
}

fn main() {
    let vec1 = Matrix2D::new([[1; 5]; 5]); // Create a new Matrix2D with zeros
    let vec2 = Matrix2D::new([[2; 5]; 5]); // Create a new Matrix2D with zeros
    let flattened = vec1.flatten(); // Flatten the matrix
    println!("{:?}", flattened); // Output: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    println!("{}", vec1.sum()); // Output: 0
    println!("{}", vec1.dot(&vec2)); // Output: 50
    println!("{:?}", vec1.transpose().data); // Output: [[
    println!("{:?}", vec1.matmul(&vec2).sum()); // Output: [[
}