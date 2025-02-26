use std::f64::consts::PI;

struct Polygon {
    num: u32,
    len: f64,
}

impl Polygon {
    fn new(num: u32, len: f64) -> Self {
        Polygon { num, len }
    }

    fn perimeter(&self) -> f64 {
        self.num as f64 * self.len
    }
    
    fn radius(&self) -> f64 {
        self.len / (2.0 * (PI / self.num as f64).sin())
    }

    fn apothem(&self) -> f64 {
        self.len / (2.0 * (PI / self.num as f64).tan())
    }

    fn area(&self) -> f64 {
        self.num as f64 * self.apothem() * self.len
    }

}

fn circle_area(r: f64) -> f64 {
    2.0 * PI * r.powi(2)
}

fn main() {
    let side_nums: [u32; 10] = [4, 8, 16, 32, 64, 128, 256, 512, 2048, 65536];
    let side_lens: [f64; 3] = [2.0, 5.0, 20.0];

    for num in side_nums.iter() {
        for len in side_lens.iter() {

            let poly = Polygon::new(*num, *len);

            let ia = circle_area(poly.apothem());
            let pa = poly.area();
            let ca = circle_area(poly.radius());

            println!(
                "
                Areas -
                Inscribed: {}
                Polygon: {}
                Circumscribed: {}
                Debug: {}
                ", 
                ia,
                pa,
                ca,
                ca > pa && pa > ia
            );

        }
    }
}