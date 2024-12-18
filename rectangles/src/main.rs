#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 20,
        height: 60,
    };

    let my_square = Rectangle::square(16);

    let area_closure = |rec: &Rectangle| rec.width * rec.height;
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!(
        "We also have that the area is {} through closure.",
        area_closure(&rect1)
    );

    println!(
        "My square has a width of {}, a height of {}, and an area of {}",
        my_square.width, my_square.height, my_square.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height 
// }
