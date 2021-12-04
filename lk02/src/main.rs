fn main() {
    let str  = "hello leikang";
    // 指向的内存地址
    let ptr = str.as_ptr();
    // 字符长度
    let len = str.len();
    println!("{:?}",str);
    println!("{:?}",ptr);
    println!("{:?}",len);
    let touple  = (300,200,199);
    let (x,y,z) = touple;
    let a = x;
    let b =y;
    let c = z;
    println!("a:{},b:{},c:{}",a,b,c);
    let touple2 = (12,34.5,67.8);
    let a_1 = touple.0;
    let a_2 = touple2.1;
    let a_3 = touple2.2;
    let a_4 = a_3;
    /// println!("touple2:{}",touple);
    println!("a_1:{},a_2:{},a_3:{},a_4:{}",a_1,a_2,a_3,a_4);

}
