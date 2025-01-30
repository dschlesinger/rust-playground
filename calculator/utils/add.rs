use test_cases::check_testcases;

pub fn add(a: i32, b: i32) -> i64 {

    // Casts a + b to i64 to prevent overflow
    let result: i64 = i64::from(a + b);

    return result
}

// Runs test cases
fn main(){

    let fn_name: String = "add";

    let test_cases =  vec![
        (1, 2),
        (3, 4),
        (9, 0),
        (-1, 2),
        (i32::MAX, i32::MAX)
    ];

    let answers = vec![
        3,
        7,
        9,
        1,
        i64::MAX -1
    ];

    let computed: Vec<i64> = test_cases.iter().map(|(a, b)| a + b).collect();

    let desc: Vec<String> = test_cases.iter_mut().zip(computed.iter_mut()).zip(answers.iter_mut()).map(
        |(((a, b), c), ans)|-> {
            format!("{}({},{})={} | {}", fn_name, a, b, c, ans)
        }
    )

    check_testcases!(computed, answers, desc)

}