use std::env;

fn main() {
    let mut args = env::args();
    let exe = args.next(); // Name of app
    let arg1 = args.next(); // Can it be NULL?!??
    println!("{:?}\n{:?}", exe, arg1);
}
