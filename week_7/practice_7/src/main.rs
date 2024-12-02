fn main() {
    let arr1:[i32;6] = [12,23,23,21,23,24];
    println!("\nArray with Data type");
    println!("array is {:?}", arr1);
    println!("array size is: {}", arr1.len());

    let arr2 = [12.4,23.5,23.6,21.7,23.7,24.8];
    println!("\nArray without Data type");
    println!("array is {:?}", arr2);
    println!("array size is: {}", arr2.len());


    let arr3:[f32;6] = [1.2,2.3,2.3,2.1,2.3,2.4];
    println!("\nArray with default type");
    println!("array is {:?}", arr3);
    println!("array size is: {}", arr3.len());

  
}
