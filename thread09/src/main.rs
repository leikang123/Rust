/**
 * 通过消息传递进行通信
 * 异步通信
 * 
 */
// 简单的生产者和消费者演示案例，主要是在主线程生成值0-9，然后在新生成线程输出
use std::{sync::mpsc::channel, thread};


 fn main() {
     let (tx,rx)= channel();
     let join_handle =thread::spawn(move || {
         while let Ok(n) = rx.recv(){
             println!("Recoved{}",n)
         }
         
     });
     for i in 0 ..10{
         tx.send(i).unwrap();
     }
     join_handle.join().unwrap();
    // println!("Hello, world!");
    
}
