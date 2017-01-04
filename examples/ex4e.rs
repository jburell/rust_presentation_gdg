#![feature(core_intrinsics)]
fn print_type_of<T>(_: &T) {
    println!("Type is: {}", unsafe { 
        std::intrinsics::type_name::<T>()
    }); 
}

fn main() {
    let a:i8 = -1;
    let b = 3u8;
    let c = a + b as i8;
    print_type_of(&c);
}   
