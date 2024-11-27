 fn main() {
    let name = "Jason Mbachu";
    let uni:&str = "Pan Atlantic University";
    let addr:&str = " Km 57 Lekki-Ekpe Exprssway, Ibeju_lekki, Lagos";
    println!("Name is: {}", name);
    println!("University is: {}, \nAdress:{} ", uni,addr );

    let department: &'static str = "Computer Science";
    let school: &'static str = " School of Science and Technology";
    println!(" Department is: {}, \nSchool: {}", department,school );

}
