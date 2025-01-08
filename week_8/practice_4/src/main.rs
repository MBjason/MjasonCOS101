fn main() {
    let name = vec!["Mary","Sam","Jason","Michael","Ronaldo"];
    let age = vec!("1","4","9","12","14");
    print!("\nAge allocation:\n");
    for i in 0..age.len()
    {
        print!("{} is {} years old\n",name[i],age[i] );
    }
    
}
