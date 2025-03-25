use std::{fs, io};
use std::collections::HashMap;

fn read_file(
    file_name: &str,          
    name_by: char,          
    split_by: &str,      
    new_line: &str        
) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new(); 

    let contents = fs::read_to_string(file_name)
        .expect("Failed to read file");

    let lines: Vec<&str> = contents.split(new_line).collect();

    for l in lines.iter() {
        let mut name_split = l.split(name_by);
        
        if let Some(key) = name_split.next() {
            let value = name_split
                .next()                  
                .unwrap_or("")
                .trim()           
                .split(split_by)      
                .map(|s| s.to_string()) 
                .collect();            

            map.insert(key.trim().to_string(), value); 
        }
    }

    map
}

fn main() {
    let recipes = read_file("data/recipes.txt", ':', ",", "\n");

    for (key, value) in &recipes {
        println!("{}: {:?}", key, value);
    }
}
