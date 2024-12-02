use std::io;

fn amorim_out(){

    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let adonis:char = input.trim().parse().expect("Failed to read input");

    if adonis >= '0' && adonis <= '9'
    {
        println!("character  {} is a digit", adonis);
    }
    else
    {
        println!("Character  {}is not a digit", adonis);
    }


}

fn main(){
    println!("Welcome, the program checks whether a character variable contains a didgit or not");
    amorim_out()
}