fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}

fn main() {
    let s = "hello world";

    let first = first_word(&s);
    println!("{first}");
}