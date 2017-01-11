#[derive(Debug)]                struct A;
#[derive(Debug, Clone)]         struct B;
#[derive(Debug, Copy, Clone)]   struct C;

fn move_to<T>(v: T) {
    let i_have_stolen_the_v = v;
}

fn main() {
    let a = A; let b = B; let c = C; let d = 3; 
    move_to(a); 
    println!("a: {:?}", a);  //<-- can't do this!
    move_to(b.clone()); 
    println!("b: {:?}", b);
    move_to(c);         
    println!("c: {:?}", c);
    move_to(d);
    println!("d: {:?}", d);
}
