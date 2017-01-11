struct Coord {
    x:f32,
    y:f32,
    z:f32,
}

fn main() {
    let c = Coord{ x: 5.0, y: 4.0, z: 3.0 };
    let Coord { x, z, .. } = c;
    println!("My x: {}, z: {}", x, z);
}
