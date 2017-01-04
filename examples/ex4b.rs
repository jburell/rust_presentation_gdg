#![feature(core_intrinsics)]
fn print_type_of<T>(_: &T) {
    println!("Type is: {}", unsafe { 
        std::intrinsics::type_name::<T>()
    }); 
}

fn main() {
    let a = 1;
    print_type_of(&a);
}   
