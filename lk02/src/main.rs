fn main() {
    let str  = "hello leikang";
    // 指向的内存地址
    let ptr = str.as_ptr();
    // 字符长度
    let len = str.len();
    println!("{:?}",str);
    println!("{:?}",ptr);
    println!("{:?}",len);
}
