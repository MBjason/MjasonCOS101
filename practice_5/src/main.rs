use std::io;

fn main() {
    //the menu
    println!("Welcome to Jacobs and Co!!");
    println!("Bellow is our menu");
    println!("We have:");
    println!("1.Pound yam for 3200");
    println!("2.Fried rice for 3000");
    println!("3. Amala and ewedu for 2500");
    println!("4. Eba and egusi for 2000");
    println!("5. White rice and stew for 2500");

    let pound_yam:i32 = 3200;
    let fried_rice:i32 = 3000;
    let amala_and_ewedu:i32 = 2500;
    let eba_and_egusi:i32 = 2000;
    let white_rice_and_stew:i32  = 2500;

    let prices =[ 3200,3000,2500,2000,2500];

    println!("What would you like to order");
    let mut order = String::new();
        io::stdin()
        .read_line(&mut order)
        .expect("Invalid input");
    println!("You odered for {}", order);
    let order :i32 =  order.trim().parse().expect("Invalid input");


    let mut quantity = String::new();
    println!("What quantity would you like?");
    io::stdin()
    .read_line(&mut quantity)
    .expect("Invalid input");
    let quantity:i32 = quantity.trim().parse().expect("Invalid input");

    let cost = order * quantity;

    if order == "pound_yam"i32{
        cost =  pound_yam * quantity
    }
    else if order == "fried_rice"i32{
        cost = quantity * fried_rice;
    }
    else if order == "amala_and_ewedu"i32 {
        cost = quantity * amala_and_ewedu;
    }
    else if order == "eba_and_egusi"i32{
        cost = quantity * eba_and_egusi;
    }            
    else if order == "white_rice_and_stew"i32{
        cost = quantity * white_rice_and_stew;
    }
        




}
