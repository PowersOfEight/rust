pub use std::fmt::Display;

pub trait OutlinePrint: Display {
    fn outline_print(&self) {
        let display_str = self.to_string();
        let length = display_str.len();
        println!("{}", "*".repeat(length + 4));
        println!("*{}*", " ".repeat(length + 2));
        println!("* {display_str} *");
        println!("*{}*", " ".repeat(length + 2));
        println!("{}", "*".repeat(length + 4));
    }
}
