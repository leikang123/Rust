use std :: thread;
use std :: sync ::{Arc,Mutex};
/**
 * 多线程环境下修改类型值
 */
fn main() {
    // 创建一个mutex值
    let vec = Arc::new(Mutex::new(vec![]));
    let mut childs = vec![];
    for i in 0 ..5 {
        let mut v = vec.clone();
        // 生成一个线程
        let t = thread::spawn(move ||{
            let mut v = v.lock().unwrap();
            v.push(i);
        });
        childs.push(t);
    }
   for c in childs {
       c.join().unwrap();
   }
   println!("{:?}",vec);
}
