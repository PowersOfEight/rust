pub mod gui {

    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            self.components
                .iter()
                .for_each(|component| component.draw());
        }
    }

    #[derive(Debug)]
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // do the drawing here using the button reference
            println!("Draw the Button: {:?}", self);
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
