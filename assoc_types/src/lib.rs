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
}
