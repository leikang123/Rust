trait T {
    // 上下两个是一样的，只不过第一个self隐藏了，Self代表类型，小写的self代表变量名
    fn method(self: Self);
    fn method2(self: &Self);
    fn method3(self: &mut Self);
}
trait T2 {
    // 直接省略变量名，写类型
    fn method4(Self);
    fn method05(&Self);
    fn method06(&mut Self);

}
// 定义一个trait函数和里面的方法
trait Share{
    fn area(self:&Self) ->f64;

}
// 定义一个结构体函数实现trait的函数体
struct C{
    rad:f64,
}
// 实现这个trait函数
impl Share for C{
    // trait函数的方法重写
    fn area(self: &Self) ->f64 {
        // self调用变量
        return std::f64::consts::PI * self.rad * self.rad;
    }
}

fn main() {

}