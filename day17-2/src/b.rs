// 结构体中使用字符串切片引用
// 这是之前留下的疑问，在此解答：
// 实例
fn main() {
    struct Str<'a> {
        content: &'a str,
    }
    let s = Str {
        content: "string_slice",
    };
    println!("s.content = {}", s.content);
}
