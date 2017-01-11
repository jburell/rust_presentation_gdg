#[derive(Debug)]
struct Point {
    x:f32,
    y:f32,
}

fn main() {
    let p = Point {x: 3.0, y: 5.2};
    println!("Value: {:?}", p);
}
