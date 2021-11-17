use std ::thread::Builder;
// 我们使用 Builder::new 方法，调用 name 和 stack_size 方法为线程分配 名称和设置堆栈大小。
// 然后我们在 my_thread 上调用 spawn，它使用构造器实例生成线程
fn main() {
    let my_thread = Builder::new().name("work thread".to_string()).stack_size(1024 *4);
    let handle = my_thread.spawn(||{
        panic!("Oops!")
    });
    let  child_status= handle.unwrap().join();

    println!("Child status:{:?}",child_status);
    // 打印结果
    // thread 'work thread' panicked at 'Oops!', main.rs:6:9
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // Child status:Err(Any { .. })
}
