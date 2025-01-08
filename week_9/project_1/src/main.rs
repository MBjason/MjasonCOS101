use std::io::Write;
fn main() {

    let mut file = std::fs::File::create("Nigerian brewery Data.doc").expect("Create Failed");
    file.write_all("Welcome to the Nigerian Breweries PLC!! The largest Brewery Company in the whole of Africa.\n".as_bytes()).expect("Create Failed");
}