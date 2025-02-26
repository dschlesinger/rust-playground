use std::time::SystemTime; 
use std::collections::HashMap;

fn main() {

    // For cached fibonacci
    // #[allow(unused_mut, unused_variables)]
    // let mut cache: HashMap<u32, u128> = HashMap::new();

    for i in 0..50 {
        let before = SystemTime::now(); 

        let a: u128 = fibonacci_uncache(i); //fibonacci_cache(i, &mut cache);

        let after = SystemTime::now(); 

        let difference = after.duration_since(before); 
        let difference = difference.expect("Time is broken"); 

        println!("Input: {} | Output: {} | Time: {:?}", i, a, difference);
    }
}

#[allow(dead_code)]
fn fibonacci_uncache(i: u32) -> u128 {
    if i == 0 {
        return 0;
    }

    if i == 1 {
        return 1;
    }

    return fibonacci_uncache(i - 1) + fibonacci_uncache(i - 2);
}

#[allow(dead_code)]
fn fibonacci_cache(i: u32, cache: &mut HashMap<u32, u128>) -> u128 {

    if let Some(&result) = cache.get(&i) {
        return result;
    }

    if i == 0 {
        return 0;
    }

    if i == 1 {
        return 1;
    }

    // Compute and store in cache
    let result = fibonacci_cache(i - 1, cache) + fibonacci_cache(i - 2, cache);
    cache.insert(i, result);

    return result;

}

#[allow(dead_code)]
fn fibonacci_dp(i: u32) -> u128 {

    if i == 0 {
        return 0;
    }

    let mut a: [u128; 2] = [0, 1];

    for _ in 0..i-1 {
        let temp = a[1];
        a[1] = a[0] + a[1];
        a[0] = temp;
    }

    return a[1];
}


