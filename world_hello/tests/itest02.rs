use world_hello;
use world_hello::tutorial;

// integration test for fn of lib

#[test]
fn it_add_point() {
    use tutorial::add_point;
    use tutorial::Point;

    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", add_point(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add_point(p3, p4));
}
