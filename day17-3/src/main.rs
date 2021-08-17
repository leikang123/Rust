mod a;

fn main() {
    // 定义变量args,变量参数名字取出并打印
    let args = std::env::args();
    // 遍历
    for arg in args {
        print!("{}", arg);
    }
}
