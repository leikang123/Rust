fn main() {
    let str  = "hello leikang";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:?}",str);
    println!("{:?}",ptr);
    println!("{:?}",len);
}
