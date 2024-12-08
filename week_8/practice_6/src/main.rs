fn main() {
    let v = vec![1,2,3,4,5,6,7,8,9];
    let u = vec![5,6,7,8,9,10,11,12];

    for index in 0..6 {
        let y:i32 = v.iter().sum();
        println!("the sum of everthing in cell one is:{}", y); 
    
        let sum:i32 = v[index] + u[index];
        println!("the sum of v and u is: {}", sum);
    }    
}
