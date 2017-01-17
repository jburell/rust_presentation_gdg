struct Coord {
    x:f32,
    y:f32,
    z:f32,
}

fn main() {
    let c = Coord{ x: 5.0, y: 4.0, z: 3.0 };
    let c2 = (42, 52);
    let Coord { x, z, .. } = c;
    let (x2, y2) = c2;
    println!("Struct x: {}, z: {}", x, z);
    println!("Tuple  x2: {}, y2: {}", x2, y2);
}
