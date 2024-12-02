use std::io

fn multiply(a:i32,b:i32){
    let multiply = a * b;

    println!("The product of A and B is = {}", multiply);
}

fn  main() {
    let mut input1 = string::new();
    println!("Enter an inputfpr parameter A: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input.trim().parse().expect("Invalid input");

    let mut input2 = string::new();
    println!("Enter an inputfpr parameter B: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let b:i32 = input.trim().parse().expect("Invalid input");

    multiply(a,b);



    
}