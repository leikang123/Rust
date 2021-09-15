fn main() {
let mut x = vec![2,3,4,5,6,7];
let value = 3;
let c = x.iter().position(|&r| r ==value).unwrap();
x.remove(c);

    println!("is {:?}",x);
}
