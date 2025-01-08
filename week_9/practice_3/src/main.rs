struct Rectangle{
    length :i32,
    breath :i32,
}
fn main() {
    let input_1 = Rectangle{
        length:12,
        breath:13,
    };
    let area = Rectangle.length*Rectangle.breath;
    println!("{:}", area);
}