use std::io;
fn main () {
    println!("Welcome to Ernest & Young (EY) global Limited");
    println!("Kindly answer the following questions");

    let mut names: Vec<String>= Vec::new();
    let mut experiences: Vec<u32> = Vec::new();

    for i in 1..=10 {
        println!("Candidate {}:", i);


        println!("What's your name ?");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        println!("Your Name is {}", name);
        let name = name.trim().to_string();

        println!("\nOkay {} tell us, how much years of experience do you have working with EY ?", name);

        let mut experience = String::new();
        io::stdin().read_line(&mut experience).expect("Failed to read line");
        println!("\nOkay {} you have had {} years of experience with EY Global limited.
        \nWe would get back to you shortly", name,experience);


        let years_of_experience:u32 = match experience.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
                return;
            }
        };

        names.push(name);
        experience.push(years_of_experience);
    }

    let mut max_experience = 0;
    let mut most_experienced_user_index = 0;

    for (index, &experience) in experiences.iter().enumerate() {
        if experience > max_experience {
           max_experience = experience;
           most_experienced_user_index = index;
        }
        
    }
    println!(" The candidate with the most experience is: {}\nName and his experience is: {}\nYears of Experience ", most_experienced_user_index,max_experience);
 


}
    






