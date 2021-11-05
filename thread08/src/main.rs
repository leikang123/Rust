use std::io::Read;
use std:: sync::RwLock;
use std:: thread;

/**
 * read只是提供对线程的读取访问权限，可以多个读取调用
 * write只是提供对线程的独占访问，将数据写入包装类型
 * 从RWLOCK实力到线程只能有一个线程写入访问权限
 */
fn main() {
    let m = RwLock::new(5);
    let c = thread::spawn(move || {
        {
            *m.write().unwrap() +=1;
        }
        let updated =*m.read().unwrap();
        updated

    });
    let updated = c.join().unwrap();
    println!("{:?}",updated);
}
