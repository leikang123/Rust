 // use std ::Vec;
fn main() {
    let mut v = Vec::<i32>::new();
    let s1 = v.len();
    let s2 = v.capacity();
    println!("s1={},s2={}",s1,s2);
    for i in 1..10{
        v.push(i);
    println!("s1={},s2={}",s1,s2);
    }
    let v1 = [1,4,7,9];
    let x2:i32 = v1.iter().sum();
    let x2_2 = v1.iter().any(|&x|x==7);
     let x2_3 = v1.iter().any(|x| *x==9);
     // let x2_4 = v1.iter().any(|x|x == 6);
    println!("x2:{}",x2);
    println!("x2_2:{}",x2_2);
    println!("x2_3:{}",x2_3);
   // println!("x2_4:{}",x2_4);
    let v2  = vec![3,4,5,6,7,8,9];
    let x3:i32 = v2.iter().sum();
    println!("x3:{}",x3);
    let x4:Vec<i32> =v2.iter().map(|x|x+1).collect();
    println!("x4:{}",x4);
    
}
