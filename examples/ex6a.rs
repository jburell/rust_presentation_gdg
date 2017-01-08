#[derive(Debug)]
struct Point { x: f32, y: f32 }

impl Point {
    fn at_x5_and_y10() -> Point {
        Point{x:5.0, y:10.0}
    }
}

trait Double {
    fn double(&self) -> Self;
}

impl Double for Point {
    fn double(&self) -> Self {
        Point{ x: self.x * 2.0, y: self.y * 2.0}
    }
}

fn main() {
    let p = Point::at_x5_and_y10();
    println!("Coordinates: {:?}, {:?}", p, p.double());
}
