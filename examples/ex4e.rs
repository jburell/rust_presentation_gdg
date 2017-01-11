use std::fs::File;

fn get_file(filename: &str) -> Result<File, std::io::Error> {
    //let f = try!(File::open(filename));
    let f = File::open(filename)?;
    Ok(f)
}

fn main() {
    match get_file("examples/ex1a.rs") {
        Ok(f) => println!("{:?}", f),
        Err(e) => println!("OOPS! {}", e),
    }
}
