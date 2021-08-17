/*fn main() {
    let x = vec![1, 23, 45, 56, 7, 8, 9, 0, 4, 34, 32, 23];
    for i in &x {
        println!("{}", i);
    }
}
 */
fn main() {
    let mut s = String::from("run");
    s.push_str("oob"); // 追加字符串切片
    s.push('!'); // 追加字符
    print!("{}", s);
}
