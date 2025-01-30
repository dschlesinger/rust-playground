use std::io;

pub fn input(line_name: String) -> i32 {

    let mut input = String::new();

    println!("{}: ", line_name);

    // Read input from the standard input
    io::stdin()
        .read_line(&mut input) // Read line into the input string
        .expect("Failed to read line"); // Handle any errors

    // Convert the input to an integer
    let ret: i32 = input.trim().parse().expect("Please type a number!");

    return ret;
}