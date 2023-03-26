pub mod fs_demo;
pub mod macro_demo;
pub mod rs_demo;

use std::ops::Add;

// demo: trait

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
