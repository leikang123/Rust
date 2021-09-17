fn main(){
    println!("- Passing a string literal"); 
    // 链接
    concatenate(" Rust ", " Programming "); 
    println!("- Passing an integer"); 
    // 链接
    concatenate(10 as i32, 1 as i32);
    
 }
 // 使用库
 use std::fmt::Display;
 fn concatenate<T:Display>(t:T, s:T){
    let result = format!("{}{}", t , s);
    println!("{}", result);
 }