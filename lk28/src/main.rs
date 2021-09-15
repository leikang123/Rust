fn main() {
let x :&str = "leikang is java";
let x2:String =String::from("Rus pro");
display_all(x);
display_string(x2);
println!("x is {}",x);
println!("x2 is {}",x2);
}
fn display_all(y_x:&str){
    println!("y_x {}",y_x);
}
fn display_string(y_string:String){
println!("y_string is {}",y_string);
}
