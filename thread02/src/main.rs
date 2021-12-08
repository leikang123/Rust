
use std::thread;

use std ::time::Duration;
fn main() {
    let x_thread = thread::spawn(move || {
        for i in 1..5{
            println!(" number is {} thread_1",i);
            thread::sleep(Duration::from_secs(1))
        }
    });
    let x_thread_2 = thread::spawn(move || {
        for i in 1..5{
            println!("numnber is {} thread_2" ,i);
            thread::sleep(Duration::from_secs(1));
        }

    });
    // join阻塞主线程的方法之一
    x_thread.join().unwrap();
    x_thread_2.join().unwrap();

    for i in 1..5 {
        println!(" number is {} main thread",i);
        thread::sleep(Duration::from_secs(1));

}
}