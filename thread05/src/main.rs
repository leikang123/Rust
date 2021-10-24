use std::thread;
use std ::sync::Arc;
fn main() {
    // 声明一个向量变量
    let nums =Arc::new(vec![1,2,3,4,5,6]);
    let mut child =vec![];
    // 遍历向量数组
    for n in 1 .. 5{
        let ns  = Arc::clone(&nums);
        // 此代码会报错
      let _i = thread::spawn( move|| {
           println!(" value is {}",ns[n])

       });
        child.push(_i)
    }
    for _i in child{
        _i.join().unwrap();
        
    }
}
