mod utils; 

use utils::add::*;

use utils::input::*;

fn main() {

    let first = input("First number".to_string());

    let second = input("Second number".to_string());

    let result = add32(first, second);
    println!("Result: {:?}", result);
}