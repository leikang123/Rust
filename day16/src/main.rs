use std::fs::File;

//主函数
fn main() {
    // 定义一个变量f 打印文件hello.txt
    let f = File::open("hello.txt");
    // 定义枚举类型match
    match f {
        // 如果找到，打印结果
        Ok(file) => {
            println!("File opened successfully.");
        }
        // 找不到就是失败
        Err(err) => {
            println!("Failed to open the file.");
        }
    }
}
