use std::fs::File;
/*这个方法不行，会报错*/

/*fn main() {
    let f = File::open("hello.txt");
    //^^^^ help: if this is intentional, prefix it with an underscore: `_file`
    if let Ok(file) = f {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }
}*/
// 如果想使一个可恢复错误按不可恢复错误处理，Result 类提供了两个办法：unwrap() 和 expect(message: &str) ：
fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 {
        Ok(i)
    } else {
        Err(false)
    }
}

fn main() {
    let r = f(10000);
    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }
}
