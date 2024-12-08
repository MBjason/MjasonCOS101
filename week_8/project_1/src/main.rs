use std::io;

fn main() {
    // Welcome message
    println!("Welcome to the Public Servant APS Level Checker!");

    // Prompt for role selection
    println!("Please choose your role from the following options:");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    // Read user input for role
    let mut role_input = String::new();
    io::stdin().read_line(&mut role_input).expect("Failed to read input");
    let role = role_input.trim().to_lowercase();

    // Ask for years of experience
    println!("How many years of experience do you have?");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience: i32 = experience_input.trim().parse().unwrap_or(0);

    // Determine APS level based on role and experience
    match role.as_str() {
        "1" | "office admin" => {
            println!("You selected Office Admin.");
            match experience {
                1..=2 => println!("Your APS level is: Intern"),
                3..=5 => println!("Your APS level is: Administrator"),
                6..=8 => println!("Your APS level is: Senior Admin"),
                9..=10 => println!("Your APS level is: Office Manager"),
                11..=13 => println!("Your APS level is: Director"),
                14..=20 => println!("Your APS level is: CEO"),
                _ => println!("Invalid experience range for Office Admin."),
            }
        }
        "2" | "academic" => {
            println!("You selected Academic.");
            match experience {
                1..=2 => println!("Your APS level is: Intern"),
                3..=5 => println!("Your APS level is: Research Assistant"),
                6..=8 => println!("Your APS level is: PhD Candidate"),
                9..=10 => println!("Your APS level is: Post Doc Researcher"),
                11..=13 => println!("Your APS level is: Senior Lecturer"),
                14..=20 => println!("Your APS level is: Dean"),
                _ => println!("Invalid experience range for Academic."),
            }
        }
        "3" | "lawyer" => {
            println!("You selected Lawyer.");
            match experience {
                1..=2 => println!("Your APS level is: Paralegal"),
                3..=5 => println!("Your APS level is: Junior Associate"),
                6..=8 => println!("Your APS level is: Associate"),
                9..=10 => println!("Your APS level is: Senior Associate 1"),
                11..=13 => println!("Your APS level is: Senior Associate 2"),
                14..=20 => println!("Your APS level is: Partner"),
                _ => println!("Invalid experience range for Lawyer."),
            }
        }
        "4" | "teacher" => {
            println!("You selected Teacher.");
            match experience {
                1..=2 => println!("Your APS level is: Placement"),
                3..=5 => println!("Your APS level is: Classroom Teacher"),
                6..=8 => println!("Your APS level is: Senior Teacher"),
                9..=10 => println!("Your APS level is: Leading Teacher"),
                11..=13 => println!("Your APS level is: Deputy Principal"),
                14..=20 => println!("Your APS level is: Principal"),
                _ => println!("Invalid experience range for Teacher."),
            }
        }
        _ => println!("Invalid role selection. Please choose a valid role."),
    }
}



















