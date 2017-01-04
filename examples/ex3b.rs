fn create_value<'a>(a:&'a mut u32) -> &'a u32 {
    *a = 2;
    return a;
}

fn main() {
    let mut a:u32 = 1;
    a = *create_value(&mut a);
    println!("Value: {}", a);
}
