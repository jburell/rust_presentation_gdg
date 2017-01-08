fn main() {
    let mut after_filter = vec!();
    let mut after_mul = vec!();
    let result:i32 = (0..10)
        .filter(|v| v % 2 == 0)
        .map(|v| { after_filter.push(v); v } )
        .map(|v| v * 3)
        .map(|v| { after_mul.push(v); v } )
        .sum();

    println!("The result is: {}, filter: {:?}, mul: {:?}",
             result,
             after_filter,
             after_mul);
}
