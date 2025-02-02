use dotenv::dotenv;
use std::env;
use tokio;
use sqlx::{PgPool, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    println!("Enter your role ( administrator, project manager,employee,customer,vendor");


    let mut role = String::new();
    io::stdin().read_line(&mut role).expect("Failed to read input");

    let role = role.trim().to_lowercase();
    let structure = get_database_structure(&role);

    println!("{}",structure);
    Ok(())


}

async fn get_database_structure(pool: &PgPool, user_role: &str) -> Result<String, splx::Error> {
    let query = match user_role {
        "administrator" => "Database structure: [Tables: users,projects,staff, customers, data_plans]",
        "project manager" => "project table structure: [columns: project_id name status deadline]",
        "employee" => "staff table structure: [columns: staff_id name department position]",
        "customer" => "customer table structure: [columns customer_id name contact orders]",
        "vendor" => "data-plan table structure columns plan_id vendor_name price data_limit]",
        _ => return ok ("invalid role please enter a valid role".to_string()),

    };
    let rows = sqlx::query(query).fetch_all(pool).await?;

    let structure: Vec<String> = rows.iter().map(|row| row.get::<String,_>(0)).collect();
    Ok(structure.join(","))


}
