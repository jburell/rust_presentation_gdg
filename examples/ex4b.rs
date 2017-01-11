struct Point {
    x:f32,
    y:f32,
}

struct NotAPoint {}

#[derive(Debug)]
enum ReturnVal { Point, NotAPoint }

fn main() {
    let p:ReturnVal = ReturnVal::NotAPoint;
    println!("Value: {:?}", p);
}
