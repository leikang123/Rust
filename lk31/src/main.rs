fn main() {
let mut x =vec![1,5,9,11,14,25];
println!("x {:?}",x);
x.push(3);
x.push(7);
println!("x {:?}",x);
x.pop();
println!("x {:?}",x);
x.remove(14);
println!("x {:?}",x);
x.contains(&5);
println!("x {:?}",x);
println!("x {:?}",x.len());
}
