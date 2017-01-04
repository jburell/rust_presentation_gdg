fn plus_two(v:i32) -> Result<i32, String> { Ok(v + 2) }
fn one_div_by(v:i32) -> Result<i32, String> { 
    if v == 0 { 
        Err("Div by 0".to_string())
    } else { 
        Ok(1 / v) 
    } 
} 
fn times_three(v:i32) -> Result<i32, String> { Ok(v * 3) } 

fn main() {
    let a = Ok(-2)
        .and_then(plus_two)
        .and_then(one_div_by)
        .and_then(times_three);
    println!("{:?}", a);
}
