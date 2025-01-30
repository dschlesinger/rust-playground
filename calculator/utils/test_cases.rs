use colored::*;

#[macro_export]
macro_rules! check_testcases {
    (computed: Vec<i64>, answers: Vec<i64>, desc: Vec<String>) => {
        if computed.len() != answers.len() {
            panic!("Vectors must be of the same length");
        }
    
        for ((info: String, c: i64), a: i64) in desc.iter_mut().zip(computed.iter_mut()).zip(answers.iter_mut()) {
            
            message: String = format!("{}={}", info, a)
            
            if c == a {
                println!("{}", message.green())
            }
            else {
                println!("{}", message.red())
            }
        }
    }
}