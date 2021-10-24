use std::thread;

fn main() {
    // 调用thread函数spawn模块
    let _child = thread::spawn(||{
        println!("thread01");
        String::from("Mush consush,shu wow!")

    });
    println!("Hello, world!");
    // 用child变量的线程调用join方法，expect获取调用join的结果方法
    let _value = _child.join().expect("");
    println!("{}",_value);
}
