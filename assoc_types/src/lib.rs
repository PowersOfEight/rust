pub mod foo;
pub mod imp;

use crate::foo::bar::Iterator;
use crate::imp::counter::Counter;

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        Some(self.val())
    }
}

mod inner {
    pub trait A {
        fn f(&self) -> usize {
            0
        }
    }

    pub trait B {
        fn f(&self) -> usize {
            1
        }
    }

    pub struct P;
    impl A for P {}
    impl B for P {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imp::counter::Counter;
    use crate::imp::disambiguator::{Animal, Dog, Human, Pilot, Wizard};
    use crate::imp::display::border::OutlinePrint;
    use crate::imp::display::newtype::Wrapper;
    use crate::imp::linear::algebra::Point;

    #[test]
    fn counter_does_count() {
        let mut cntr = Counter::new();

        for _ in 0..5 {
            match cntr.next() {
                Some(x) => println!("Counted to {x}"),
                None => panic!("No value returned from counter"),
            }
        }
    }

    #[test]
    fn test_get() {
        let mut cntr = Counter::starting_at(1);

        for i in 1..5 {
            assert_eq!(i, cntr.get());
            // treat like a lcv
            let _ = cntr.next();
        }
    }

    #[test]
    fn test_add_point() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }

    #[test]
    fn test_disambiguation() {
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();
    }

    // In general, it works like this
    // <Type as Trait>::function_name(receiver_if_method, next_arg, ...);

    #[test]
    fn test_no_ref_disambiguation() {
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }

    #[test]
    fn test_display_works_on_point() {
        let my_point = Point { x: 7, y: 11 };
        my_point.outline_print();
    }

    #[test]
    fn test_newtype_wrapper() {
        let wrapper = Wrapper(vec![
            String::from("foo"),
            String::from("bar"),
            String::from("baz"),
        ]);

        assert_eq!("[foo, bar, baz]", wrapper.to_string());
    }

    #[test]
    fn default_wrapper() {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {w}");
    }

    #[test]
    fn quiz_3() {
        use crate::inner::{B, P};
        println!("{}", P.f());
    }
}
