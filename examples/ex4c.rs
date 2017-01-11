#[derive(Debug, Clone)]
enum ReturnVal { Point{ x:f32, y:f32 }, NotAPoint }

fn get_val(input:f32) -> ReturnVal {
    if input > 1.0 { 
        ReturnVal::Point{ x: input, y: input} 
    } else { 
        ReturnVal::NotAPoint 
    }
}

fn main() {
    let p:ReturnVal = get_val(1.5);
    println!("Value: {:?}", p);
}
