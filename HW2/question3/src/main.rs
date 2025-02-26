use std::io;

fn main() {

    let mut input =String ::new(); 
    io::stdin().read_line(&mut input).expect("Failed to read line"); 
    let input = input.trim(); 
    let number: u8 = input.parse().expect("Not a good number!"); 

    println!("sum of cubes: {}", sumcube(number));

}

#[allow(dead_code)]
fn sumcube(x: u8) -> u32 {
    let mut sum: u32 = 0;
    for i in 1..(x as u32) {
        sum += i.pow(3);
    }

    return sum;
}
