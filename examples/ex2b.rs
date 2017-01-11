#[derive(Debug)]
struct A { b: B }
#[derive(Debug, Clone)]
struct B { c: i32 }

fn you_can_borrow_this(a: &A) {
    let mut x = (*a).b.clone();
    x = B{ c: x.c * 2 };
    println!("x: {:?}", x);
}

fn main() {
    let y = A{ b: B{ c: 3 } };
    you_can_borrow_this(&y);
    println!("y: {:?}", y);
}
