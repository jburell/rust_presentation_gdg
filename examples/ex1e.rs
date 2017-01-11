fn create_value_ref<'a>() -> &'a u32 {
    let a:u32 = 2;
    return &a; // Not allowed!
}

fn create_value_move() -> u32 {
    let a:u32 = 3;
    return a;
}

fn main() {
    let a:&u32 = create_value_ref();
    println!("Value: {}", *a);

    let b:u32 = create_value_move();
    println!("Value: {}", b);
}
