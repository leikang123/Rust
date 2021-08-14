// 定义结构体
struct abc{
    // 属性
    width: u32,
    height: u32,
}
// 实现结构体
impl abc {
    // 结构体方法
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}
// 主方法
fn main() {
    // 定义变量rect并打印
    let rect = Rectangle::create(30, 50);
    println!("{:?}", rect);
}


