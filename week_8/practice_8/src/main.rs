fn main() {
    let mut jason:(&str, i32, &str, i64) = ("Jason", 17, "Mbachu", 2007);
    println!("My first name is {:?}", jason.0);
    println!("I am {:?}", jason.1);
    println!("My last name is {:?}", jason.2);
    println!("I was born in {:?}", jason.3);

    jason.1 = 18;
    jason.3 = 2006;

    println!("Changed tuple {:?}", jason);
}
