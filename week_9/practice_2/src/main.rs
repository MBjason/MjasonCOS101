use std::io::Read;
use std::io::Write;
//Read write and creating a file 
fn main(){
    let mut file = std::fs::File::create("jason.txt").expect("create failed");
    file.write_all("Hi I'm Jason and this is my first .txt file ever on Rust!!!".as_bytes());

    let mut file = std::fs::File::open("jason.txt").unwrap();
    let mut contents   = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}