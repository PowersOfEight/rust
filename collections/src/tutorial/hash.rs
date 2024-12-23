use colored::Colorize;


pub fn hash_map_demo() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        let colored = format!{"[\"{key}\":"}.cyan();
        let colored = format!{"{colored}{value}]"}.purple();
        println!("{colored}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{map:?}");

    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    scores.entry(String::from("Orange")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // lets edit the entries
    // we'll make a simple program that counts the multiplicities
    // of each distinct word in a string

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    let result_string = format!("Resultant map: {map:?}").on_yellow().red();

    println!("{result_string}");
    
}