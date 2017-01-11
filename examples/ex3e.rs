#[derive(Debug)]
struct A { b: B }
#[derive(Debug, Clone)]
struct B { c: i32 }

fn you_can_borrow_this(a: &A) {
    let mut b = (*a).b.clone();
    b = B{ c: b.c * 2 };
    println!("b: {:?}", b);
}

fn main() {
    let a = A{ b: B{ c: 3 } };
    you_can_borrow_this(&a);
    println!("a: {:?}", a);
}
