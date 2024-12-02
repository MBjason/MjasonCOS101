use std::io;

fn main() {
    loop {
        println!("Select a calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Go Away");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                println!("Enter height:");
                let height = read_float();
                println!("Enter base1:");
                let base1 = read_float();
                println!("Enter base2:");
                let base2 = read_float();
                let area = area_of_trapezium(height, base1, base2);
                println!("Area of Trapezium: {}", area);
            }
            2 => {
                println!("Enter diagonal1:");
                let diagonal1 = read_float();
                println!("Enter diagonal2:");
                let diagonal2 = read_float();
                let area = area_of_rhombus(diagonal1, diagonal2);
                println!("Area of Rhombus: {}", area);
            }
            3 => {
                println!("Enter base:");
                let base = read_float();
                println!("Enter altitude:");
                let altitude = read_float();
                let area = area_of_parallelogram(base, altitude);
                println!("Area of Parallelogram: {}", area);
            }
            4 => {
                println!("Enter length of the side:");
                let side = read_float();
                let area = area_of_cube(side);
                println!("Area of Cube: {}", area);
            }
            5 => {
                println!("Enter radius:");
                let radius = read_float();
                println!("Enter height:");
                let height = read_float();
                let volume = volume_of_cylinder(radius, height);
                println!("Volume of Cylinder: {}", volume);
            }
            6 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn area_of_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    (height / 2.0) * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * side.powi(2)
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius.powi(2) * height
}

fn read_float() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse::<f64>().unwrap_or(0.0)
}
