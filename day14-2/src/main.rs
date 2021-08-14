// 定义option枚举类
enum Option<T> {
    Some(T),
    None,
}
fn main() {
    // option 类型枚举类，定义变量opt类型，并打印hello
    // let opt = Option::Some("hello");
    let opt: Option<&str> = Option::None;
    // 实现枚举类型
    match opt {
        // 分之-》返回值
        Option::Some(something) => {
            println!("{}", something);
        }
        // 分之-》返回值
        Option::None => {
            println!("opt is nothing")
        }
    }
}
