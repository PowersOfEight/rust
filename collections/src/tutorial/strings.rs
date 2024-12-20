use colored::*;

pub fn string_demo() {
    // let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();
    let other_way = "initial contents".to_string();

    if s.eq(&other_way) {
        let s = s.red();
        let other_way = other_way.green();
        println!("\"{s}\"\n\"{other_way}\"");
    }

    // We can push to a string in rust:
    let mut s = String::from("foo");
    s.push_str("bar");
    dbg!(s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is \"{}\"", s2.red());
    println!("s1 -> \"{}\"", s1.blue());


    let mut s = String::from("lu");
    s.push('l');
    s.push('z');

    println!("I do it for the -> \"{}\"", s.purple());

    // We can concatenate with the plus operator but...
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // <~~ note that s1 has been moved here

    println!("\"{}\"", s3.green().underline());

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s1 = s1 + "-" + &s2 + "-" + &s3;

    let s = format!("{s1}-{s2}-{s3}");
    println!("\"{}\"", s.yellow());

    //this won't compile |
                     //  V
    // let s1 = String::from("hello");
    // let h = s1[0];//<~~ this won't work because
    // the rust compiler can't tell what type of 
    // value you want.  Do you want a byte, a 
    // utf-8 scalar value or a character?

    // lets try some character stuff
    let s = String::from("Some random words");

    for c in s.chars() {
        let color_char = format!("'{c}'").purple();
        println!("{color_char}");
    }

    for c in "Зд".chars() {
        let why_not = format!("'{c}'").on_blue().magenta();
        println!("{why_not}");
    }
    
    for b in "Зд".bytes() {
        let current_byte = format!("\"[{b}]\"").green();
        println!("{current_byte}");
    }
    

}