use std::fmt::{Display, Formatter, Result};

pub struct Wrapper(pub Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
