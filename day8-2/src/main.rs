fn main() {
    // 定义字符串变量s1
    let s1 = String::from("hello");
   // 定义两个变量s2,len,两个变量的值在后面方法里的参数s1
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}
// 定义方法病初始化
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
