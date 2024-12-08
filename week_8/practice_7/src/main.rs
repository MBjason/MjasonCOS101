fn main () {
    let jason: (&str, f64, i32) = ("Kaka", 22.5,9);
    println!("The tuples are {:?}", jason);

    println!("the first data is {:?}", jason.0);
    println!("The second data is {:?}", jason.1);
    println!("The third element is {:?}", jason.2);
}