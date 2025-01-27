fn main() {
    let x = 42;
    let mut x = x;
    x = 51;
    println!("x: {}", x);
    let mut y = &x;
    println!("y: {}", y);
    let z = 51;
    y = &z;
    println!("*y: {}", *y);
    
    let mut a = Box::new(y);

    println!("*a: {}", *a);
}
