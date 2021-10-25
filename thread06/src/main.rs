
use std :: thread;
use std :: sync :: Mutex;
/**
 * 单线程环境下修改变量值
 */
fn main() {
    // 声明线程互斥一个
    let m = Mutex:: new (1);
    // 创建线程
    let c = std::thread::spawn(move || {
        // 互斥线程枷锁
        *m.lock().unwrap() +=1;
        // 互斥线程更新
        let update  = *m.lock().unwrap();
        update
    });
    // 线程结束并打印
    let update = c.join().unwrap();
    println!("{:?}",update);
}
