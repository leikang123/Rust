fn main() {
     
    println!("Number is 12:{}",test(12));
    println!("Number is 9 :{}",test(9));
    println!("Number is 8 :{}",test(8));
    println!("number is 23:{}",test(23));
}
fn test(a:i32)-> i32{
if a % 3 == 0 && a % 4 == 0 {
0
}
else if a % 3 == 0 && a % 4 != 0 {
1
}
else if a % 3 != 0 && a % 4 == 0{
2
}
else {
-1
}
}
