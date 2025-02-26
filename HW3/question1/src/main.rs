use std::f64::consts::PI;

enum Shape {
    Sphere(f64),
    Cuboid(f64, f64, f64),
    Pyramid(f64, f64, f64),
}

impl Shape {
    fn new(t: String, l: f64, w: Option<f64>, h: Option<f64>) -> Self {
        match t.as_str() {
            "Sphere" => Shape::Sphere(l),
            "Cuboid" => Shape::Cuboid(l, w.unwrap(), h.unwrap()), // Use default value if None
            "Pyramid" => Shape::Pyramid(l, w.unwrap(), h.unwrap()), // Use default value if None
            _ => panic!("Invalid shape type: {}", t),
        }
    }

    fn surface_area(&self) -> f64 {
        match self {
            Shape::Sphere(r) => 4.0 * PI * r.powi(2),
            Shape::Cuboid(l, w, h) => 2.0 * (l * w + l * h + w * h),
            Shape::Pyramid(l, w, h) => (l * w) + (l * ((w / 2.0).powi(2) + h.powi(2)).sqrt()) + (w * ((l / 2.0).powi(2) + h.powi(2)).sqrt()),
        }
    }

    fn volume(&self) -> f64 {
        match self {
            Shape::Sphere(r) => (4.0 / 3.0) * PI * r.powi(3),
            Shape::Cuboid(l, w, h) => l * w * h,
            Shape::Pyramid(l, w, h) => (l * w * h) / 3.0,
        }
    }

    fn extend_y(&self) -> Self {
        match self {
            Shape::Sphere(r) => Shape::new("Sphere".to_string(), r * 2.0, None, None),
            Shape::Cuboid(l, w, h) => Shape::new("Cuboid".to_string(), l * 2.0, Some(*w), Some(*h)),
            Shape::Pyramid(l, w, h) => Shape::new("Pyramid".to_string(), l * 2.0, Some(*w), Some(*h)),
        }
    }

    fn verify(&self) -> bool {
        match self {
            Shape::Sphere(r) => *r > 0.0,
            Shape::Cuboid(l, w, h) | Shape::Pyramid(l, w, h) => *l > 0.0 && *w > 0.0 && *h > 0.0,
        }
    }
}

fn check(obj: Shape) {

    if obj.verify() {
        println!("obj Verify Good");
        println!("obj Surface Area: {}", obj.surface_area());
        println!("obj Volume: {}", obj.volume());
        println!("obj Double y Volume: {}", obj.extend_y().volume());
    }

    else {
        println!("obj Verify Bad");
    }

    println!("\n");
}

fn main() {

    let sphere1 = Shape::new("Sphere".to_string(), 3.0, None, None);
    let sphere2 = Shape::new("Sphere".to_string(), -3.0, None, None);

    println!("Sphere");
    check(sphere1);
    check(sphere2);
    

    let cube1 = Shape::new("Cuboid".to_string(), 3.0, Some(4.0), Some(5.0));
    let cube2 = Shape::new("Cuboid".to_string(), 3.0, Some(-2.0), Some(1.0));

    println!("Cube");
    check(cube1);
    check(cube2);
    
    let pyramid1 = Shape::new("Pyramid".to_string(), 3.0, Some(4.0), Some(5.0));
    let pyramid2 = Shape::new("Pyramid".to_string(), 3.0, Some(4.0), Some(-5.0));

    println!("Pyramid");
    check(pyramid1);
    check(pyramid2);

}
