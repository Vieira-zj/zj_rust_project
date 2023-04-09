pub mod apps;
pub mod config;
pub mod tutorial;

pub use self::apps::webserver::{app_v1, app_v2};

// fn for integration test

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
