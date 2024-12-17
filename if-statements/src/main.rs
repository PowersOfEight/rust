fn recurse (i: u32) -> () {
    if i == 0 {
        println!("blastoff!");
        return 
    }
    println!("{i}");
    recurse (i - 1)
}

fn functional_map(p: bool) -> u32 {
    if p {
        1
    } else {
        0
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", functional_map(true));
    recurse(25);
}
