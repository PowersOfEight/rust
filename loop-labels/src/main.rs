fn increment(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        increment(&mut count);
    }
    println!("End count = {count}");
}
