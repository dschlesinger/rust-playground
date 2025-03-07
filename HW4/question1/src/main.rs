use std::ops::{Neg, Mul, Add};

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T:Copy + Neg<Output = T> + Mul<Output = T> + Add<Output = T> + From<i8>> Point<T> {
    // Precompute trig values

    fn cos90(&self) -> T { // cos(90) == 1
        T::from(0)
    }
    
    fn sin90(&self) -> T { // sin(90) == 0
        T::from(1)
    }

    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn clockwise(&self) -> Self {
        Point { 
            x: (self.x * self.cos90()) + (self.y * self.sin90()),
            y: (self.x * -self.sin90()) + (self.y * -self.cos90()),
        }
    }

    fn counterclockwise(&self) -> Self {
        Point { 
            x: (self.x * self.cos90()) + (self.y * -self.sin90()),
            y: (self.x * self.sin90()) + (self.y * self.cos90()),
        }
    }
}

fn main() {
    let ipoint = Point::new(1 as i64, 0 as i64);
    let fpoint = Point::new(1 as f32, 1 as f32);
    let ipoint_clockwise = ipoint.clockwise();
    let fpoint_counterclockwise = fpoint.counterclockwise();
    
    println!("Integer Point: ({}, {})", ipoint.x, ipoint.y);
    println!("Integer Point Rotated Clockwise: ({}, {})", ipoint_clockwise.x, ipoint_clockwise.y);
    println!("Integer Point: ({}, {})", fpoint.x, fpoint.y);
    println!("Rotated Counterclockwise: ({}, {})", fpoint_counterclockwise.x, fpoint_counterclockwise.y);

    // Test cases
    assert_eq!(Point::new(1, 1).clockwise(), Point::new(1, -1));
    assert_eq!(Point::new(-1, -1).counterclockwise(), Point::new(1, -1));
}