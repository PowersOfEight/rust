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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::imp::counter::Counter;
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
}
