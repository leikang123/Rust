use std::thread;
use std::fmt::Display;
fn main() {
    let v = vec![1,2,3,4,5];
    // 调用thread函数spawn模块
    let child_1= thread::spawn(move ||{
        println!("{:?}",v);
    });
   child_1.join().unwrap();
}
