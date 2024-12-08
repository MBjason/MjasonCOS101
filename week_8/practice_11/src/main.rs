fn main() {
    let numbers = [1,2,3,4,5,6,7,8];
    println!("Original array is: {:?}", numbers);

    let slice1 = &numbers[0..6];
    println!("2nd and 3rd element sliced are {:?}", slice1);
    println!("Hello, world!");
}
