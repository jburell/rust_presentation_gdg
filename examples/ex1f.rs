#![feature(box_syntax)]
fn create_value() -> Box<u32> {
    let a:Box<u32> = box 2;
    return a;
}

fn main() {
    let a:u32 = *create_value();
    println!("Value: {}", a);
}
