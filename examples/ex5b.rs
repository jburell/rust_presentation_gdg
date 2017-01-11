fn test_pattern(v: i32) -> () {
    match v {
        a if a > 4 => println!("5 or bigger!"),
        1 ... 2    => println!("1..2"),
        3 | 4      => println!("3|4"),
        _          => println!("Don't know!"),
    }
}

fn main() {
    let _:Vec<()> = (0..)
        .take(6)
        .map(test_pattern)
        .collect();
}
