fn main() {
let mut x_y = vec![2,3,4,5,6,7,9];
println!("x_y is value {:?}",x_y);
for i in x_y.iter_mut(){
  *i *=2;
}
    println!("x_y is value {:?}",x_y);
}
