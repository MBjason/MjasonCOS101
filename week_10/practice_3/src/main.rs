fn main(){
    let v = vec![20,30,40,50];
    let v2 = v;
    let v2_return = display(v2);
    println!(" In main v is{:?}", v);
}

fn display(v:Vec<i32>)->Vec<i32>{
    println!(" Inside display {:?}", v);
    
}