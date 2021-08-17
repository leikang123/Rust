// 引入io的包，进行iO操作
use std::io::stdin;

fn main() {
    let mut str_buf = String::new();

    stdin()
        .read_line(&mut str_buf)
        .expect("Failed to read line.");

    println!("Your input line is \n{}", str_buf);
}
