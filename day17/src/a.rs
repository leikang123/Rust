// 结构体和枚举类定义范型
// 结构体定义范型方法
struct Point<T> {
    // 枚举类型两种型，x,y
    x: T,
    y: T,
}
// 实现结构体的范型 impl 关键字的后方必须有 <T>，因为它后面的 T 是以之为榜样的。但我们也可以为其中的一种泛型添加方法：
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// 主类
fn main() {
    let p = Point { x: 1, y: 2 };
    println!("p.x = {}", p.x());
}
// impl 块本身的泛型并没有阻碍其内部方法具有泛型的能力：
/*impl<T, U> Point<T, U> {
fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
        x: self.x,
        y: other.y,
    }
}*/
