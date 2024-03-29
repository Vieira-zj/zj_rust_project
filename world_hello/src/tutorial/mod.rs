pub mod calculator;
pub mod download;
pub mod fsdemo;
pub mod linkedlist;
pub mod macrodemo;
pub mod rsdemo;
pub mod utilsdemo;

// demo: trait

use std::ops::Add;

#[derive(Debug)]
pub struct Point<T: Add<T, Output = T>> {
    pub x: T,
    pub y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

pub fn add_point<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}
