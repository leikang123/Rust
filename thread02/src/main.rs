
use std::thread;

fn main() {
    //调用thread的spaw函数模块，必包参数，单独线程为子线程
   let _child = thread::spawn(|| {
        println!("Thread!");
       // "Mush concurent, sush wow!".to_string()
       String::from("Mush concurrent,sush wow!")

    });
    // 先执行主线程，后执行子线程
    println!("Hello, world!");
}
