use std::{thread,time::Duration};
fn main() {
    // 创建一个线程调用无参数闭包
    let x_thread = thread::spawn(move ||{
        for i in 1 ..10 {
            println!("hi number {} from the spawed thread_1 ",i);
            thread::sleep(Duration::from_secs(2));
            //thread::sleep(dur:Duration::from_millis(1));
        }
    });
    let x_thread_2 = thread::spawn(move || {
        for i in 1..5 {
            println!("number is {} thread_2",i);
            thread ::sleep(Duration::from_secs(1));
        }
    });
    for i in 1..5 {
        println!("main is {} thread",i);
        thread::sleep(Duration::from_secs(2));
    }
    x_thread.join();
    x_thread_2.join();
}
