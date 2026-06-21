use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

use crate::imp::display::border::OutlinePrint;

impl OutlinePrint for Point {}

/*
* If one wants to see what kind ofshenigans one can get into wth default type parameters, read the
* hover description for the Add operator (which overloads the + operator for custom type)
*
core::ops::arith

pub trait Add<Rhs = Self>

The addition operator `+`.

Note that `Rhs` is `Self` by default, but this is not mandatory. For
example, [`std::time::SystemTime`](https://doc.rust-lang.org/stable/core/std/time/struct.SystemTime.html) implements `Add<Duration>`, which permits
operations of the form `SystemTime = SystemTime + Duration`.

# Examples

## `Add`able points

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
           Point { x: 3, y: 3 });

## Implementing `Add` with generics

Here is an example of the same `Point` struct implementing the `Add` trait
using generics.

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// Notice that the implementation uses the associated type `Output`.
impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
           Point { x: 3, y: 3 });
*
*
* As you can see, the Add trait uses a default type parameter of self, which
* allows a developer to bypass putting a type parameter for theexpected type
* of the right hand operand
*/

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
