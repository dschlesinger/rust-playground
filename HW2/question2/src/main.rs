fn main() {
    let mut arr: [u8; 181] = [0; 181];
    arr[1] = 1;

    for i in 2..arr.len() {
        arr[i] = arr[i-1] + arr[i-2];
    }

    for index in arr.iter().enumerate() {
        if index.0 > 149 {
            println!("Fibonacci of {} is {}", index.0, index.1);
        }
    }
}
