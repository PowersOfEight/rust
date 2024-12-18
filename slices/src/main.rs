fn first_word(s: &str) -> &str {
    for (i, &item) in s
        .as_bytes()
        .iter()
        .enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] 
}

fn main() {
    let mut s = String::from("hello world");
    
    let hello: &str = first_word(&s);
    let world: &str = first_word(&s[hello.len()..].trim_start());

    println!("{hello} {world}");
    
    s.clear();
}
