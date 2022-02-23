use std::thread;
use std::time;

fn main() {
    // 线程开始时间

    let start  = time::Instant::now();
    // 创建一个线程
    let handle = std::thread::spawn(move || {
        // 变量panic时间100秒
        let panic =time::Duration::from_millis(100);
        thread::sleep(panic.clone());  
    });
    //
    handle.join().unwrap();
    let finsh = time::Instant::now();
    println!("{:02?}",finsh.duration_since(start));
}
