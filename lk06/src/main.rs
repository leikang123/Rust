fn main() {
    // 定义数组
    let double = |x| x *2;
    let a = 6;
    let b = double(a);
    println!("b is {}",b);
    let bc = |x,y| {
        let z=x+y;
        z * b;
    };
    let sm = bc(2,4);
    print!("sm is {:?}",sm);
}
