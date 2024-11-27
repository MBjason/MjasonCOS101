use std::io;
fn main() {
    let _pound_yam = 3200;
    let _fried_rice = 3000;
    let _amala_and_ewedu = 2500;
    let _eba_and_egusi = 2000;
    let _white_rice_and_stew = 2500;
    println!("Welcome to jacobs and co fast food");

    println!("We have
    pound yam for 3200
    Fried rice for 3000
    Amala and ewedu for 2500
    Eba and egusi for 2000
    White rice and stew for 2500");


    loop{ 
        println!("\nWhat would you like to order");
        let mut order = String::new();
            io::stdin()
            .read_line(&mut order)
            .expect("Invalid input");
        println!("You odered for {}", order);
    
    

        println!("How would you like to order? (quantity)");
       
        let mut quantity = String::new();
            io::stdin()
            .read_line(&mut quantity)
            .expect("Invalid input");
        if order == "_pound_yam"{
            let cost = quantity * _pound_yam;
        } 
        else if order == "_fried_rice"{
            let cost = quantity * _fried_rice;
        }
        else if order == "_amala_and_ewedu" {
            let cost = quantity * _amala_and_ewedu;
        }
        else if order == "_eba_and_egusi"{
            let cost = quantity * _eba_and_egusi;
        }            
        else if order == "_white_rice_and_stew"{
            let cost = quantity * _white_rice_and_stew;
        }
        else{
            println!("Invalid input");
        }
        println!("You are ordering for {} of {}", quantity,order);
        let quantity : f64 = order.trim().parse().expect("Invalid input");


        
        println!("your order is {} naira in total", cost);
    }
           

        
    
    
    

}



