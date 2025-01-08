fn main(){
    let v = vec![12,13,14,15];
    print_vector(v);
    println!("{:?}", v[0]);
}

fn print_vector(x:Vec<i32>){
    println!("{:?}", x);
}
// This code didnt run( flagged an error) because the value of v has been replaced with x 