// 导包
use std::fs;
use std::io::prelude::*;
/*fn main() {
    // 定义文本读取磁盘文件,根据系统不同，Mac的格式//。。。，/。。
    let text = fs::read_to_string("//Volumes/THU/lk.txt").unwrap();
    println!("{}", text);
}*/
/*fn main() {
    let content = fs::read("/Volumes/THU/a.txt").unwrap();
    print!("{:?}", content);
}*/

// 一次性读取
//std::fs 模块中的 File 类是描述文件的类，可以用于打开文件，再打开文件之后，
// 我们可以使用 File 的 read 方法按流读取文件的下面一些字节到缓冲区（缓冲区是一个 u8 数组），读取的字节数等于缓冲区的长度。
fn main() {
    let mut buffer = [0u8; 5];
    let mut file = fs::File::open("//Volumes/THU/lk.txt").unwrap();
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
}
