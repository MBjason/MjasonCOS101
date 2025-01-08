struct Laptop {
    IBM :i64,
    HP :i64,
    Toshiba :i64,
    Dell :i64,

}
fn main() {
    let  cost = Laptop {
        IBM: 755000*3,
        HP: 650000*3,
        Toshiba: 550000*3,
        Dell: 850000*3,
    };
    println!("The total cost  of the 10 different PC models is :{} ", 
        sum(&cost)
    );   
}
fn sum(laptop :&Laptop) -> i64 {
    laptop.IBM + laptop.HP + laptop.Toshiba + laptop.Dell


}