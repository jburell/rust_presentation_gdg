#[derive(Debug/*, Copy, Clone*/)]
struct A;

fn move_to(a: A) {
    let i_have_stolen_the_a = a;
}

fn copy_into(a: i32) {
    let my_local_copy = a;
}

fn main() {
    let a = A;
    let b = 3;
    copy_into(b);
    println!("b: {}", b);
    move_to(a);
    println!("a: {:?}", a);
}
