use std::io;
fn main(n:Option<&char>) {
    println!("Elemen of Vector {:?}", n);
    
}
fn main(){


    let v = vec!['c','d','f','g','h','j','t','r'];
    let mut input1 = String::new();

    println!("Enter an index value bettwen (0-7)");
    io::stdin()
    .read_line(&mut input1
    .expect("failed to read input");

    let index:usize = input.tim().parse().expect("Failed to read line");

    let ch: Options<&char> = v.get(index);
    value(ch);
}