fn  main() {
    let arr:[i32;4] = [14,45,43,22];
    println!("array is {:?}", arr);
    println!("Array size is :{}", arr.len());

    for val in arr.iter(){
        println!("value is :{}", val);
    }

}