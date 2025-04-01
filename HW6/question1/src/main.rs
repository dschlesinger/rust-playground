use std::collections::HashMap;

fn read_file(
    file_name: &str,
    name_by: char,
    split_by: &str,
    new_line: &str,
) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let contents = fs::read_to_string(file_name).expect("Failed to read file");

    let lines: Vec<&str> = contents.split(new_line).collect();

    for l in lines.iter() {
        if l.trim() == "" {
            continue;
        }

        let mut name_split = l.split(name_by);

        if let Some(key) = name_split.next() {
            let value = name_split
                .next()
                .unwrap_or("")
                .trim()
                .split(split_by)
                .map(|s| s.to_lowercase().to_string())
                .filter(|s| !s.is_empty()) 
                .collect();

            map.insert(key.trim().to_lowercase().to_string(), value);
        }
    }

    map
}

fn person_likes(
    person: String,
    recipe: String,
    people: &HashMap<String, Vec<String>>,
    recipes: &HashMap<String, Vec<String>>,
    categories: &HashMap<String, Vec<String>>,
) -> bool {
    let person_categories = people.get(&person.to_lowercase()).expect("Person not found");
    let recipe_ingredients = recipes.get(&recipe.to_lowercase()).expect("Recipe not found");

    let mut recipe_categories: Vec<String> = vec![];

    // Get recipe ingredients categories
    for ing in recipe_ingredients.iter() {
        for (cat, cat_ings) in categories {
            if cat_ings.contains(ing) {
                recipe_categories.push(cat.to_string());
                break;
            }
        }
    }

    let mut sum: i32 = 0;
    let total: i32 = recipe_categories.len() as i32;

    // Check percent
    for rec_cat in recipe_categories.iter() {
        if person_categories.contains(rec_cat) {
            sum += 1;
        }
    }

    let percent: i32 = ((sum as f32 * 100.0) / total as f32).ceil() as i32;

    // Return
    percent >= 60
}

fn find(
    person: String,
    people: &HashMap<String, Vec<String>>,
    recipes: &HashMap<String, Vec<String>>,
    categories: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut found: Vec<String> = vec![];

    for (key, _) in recipes {
        if person_likes(person.clone(), key.clone(), people, recipes, categories) {
            found.push(key.clone());
        }
    }

    found
}

fn counter<T: Eq + std::hash::Hash + Clone>(list: &Vec<T>) -> HashMap<T, i32> {
    let mut result: HashMap<T, i32> = HashMap::new();

    for item in list.iter() {
        let count = result.entry(item.clone()).or_insert(0);
        *count += 1;
    }

    result
}

fn popular_recipes(
    people: &HashMap<String, Vec<String>>,
    recipes: &HashMap<String, Vec<String>>,
    categories: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut recipe_hits: Vec<String> = vec![];

    for p in people.keys() {
        recipe_hits.extend(find(p.clone(), people, recipes, categories));
    }

    let hits_counter = counter(&recipe_hits);

    let mut sorted_by_count: Vec<(&String, &i32)> = hits_counter.iter().collect();
    sorted_by_count.sort_by(|&(key1, count1), &(key2, count2)| {count2.cmp(count1).then_with(|| key2.cmp(&key1))});

    sorted_by_count.iter().take(3).map(|&(key, _)| key.clone()).collect();
}

fn load() -> (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) {
    let recipes = read_file("data/recipes.txt", ':', ",", "\n");
    let categories = read_file("data/categories_ingredients.txt", ':', ", ", "\n");
    let people = read_file("data/people_categories.txt", ':', ",", "\n");

    (recipes, categories, people)
}

fn cmd(
    people: &HashMap<String, Vec<String>>,
    recipes: &HashMap<String, Vec<String>>,
    categories: &HashMap<String, Vec<String>>,
) {
    let mut input = String::new();

    print!("\npopr: popular recipes\np: person likes\npr: person recipe combo\nYour cmd: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();
    
    if input == "popr".to_string() {
        let pr = popular_recipes(&people, &recipes, &categories);
        println!("Top Recipes:");
        for r in pr.iter() {
            println!("{}", r);
        }
    }

    else if input == "p" {
        let mut person = String::new();

        print!("Person: ");

        io::stdin()
            .read_line(&mut person)
            .expect("Failed to read line");

        let prefrences = find(person.trim().to_lowercase(), &people, &recipes, &categories);

        println!("{:?}", prefrences);
        
    }

    else if input == "pr".to_string() {
        let mut person = String::new();

        print!("Person: ");

        io::stdin()
            .read_line(&mut person)
            .expect("Failed to read line");

        let mut recipe = String::new();

        print!("Recipe: ");

        io::stdin()
            .read_line(&mut recipe)
            .expect("Failed to read line");

        let likes = person_likes(person.trim().to_lowercase(), recipe.trim().to_lowercase(), &people, &recipes, &categories);

        println!("{}", likes)

    }
}

fn main() {
    let (recipes, categories, people) = load();

    let p_l_r: bool = person_likes("Sylvie Wilson".to_string(), "Recipe987".to_string(), &people, &recipes, &categories);
    let likes: Vec<String> = find("Ryker McPherson".to_string(), &people, &recipes, &categories);

    println!("{}", p_l_r);

    cmd(&people, &recipes, &categories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_likes() {
        let (recipes, ingredients, people) = load();
        assert_eq!(person_likes("Ryker McPherson".to_string(), "Recipe987".to_string(), &people, &recipes, &ingredients), true);
        assert_eq!(person_likes("Sylvie Wilson".to_string(), "Recipe987".to_string(), &people, &recipes, &ingredients), false);
    }
}
