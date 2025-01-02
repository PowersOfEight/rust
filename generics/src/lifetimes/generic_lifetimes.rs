use std::fmt::{Display, Formatter};

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub struct Announcement<'a> {
    ann: &'a str,
}

impl Display for Announcement<'a,&'a str, E>{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<T,E> {
        todo!()
    }
}
pub fn main() {
    let s1 = String::from("long string is a l-o-n-g string");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);
    println!("The longest string is \"{result}\"");
}


// &i32         // a reference
// &'a i32      // a reference with an explicit lifetime
// &'a mut i32  // a mutable reference with an explicit lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn shortest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//     if x.len() < y.len() {
//         x
//     } else {
//         y
//     }
// }