fn create_value<'a>() -> &'a u32 {
    let a:u32 = 2;
    return &a;
}

fn main() {
    let a:&u32 = create_value();
    println!("Value: {}", a);
}
