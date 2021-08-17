// use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
// 创建一个文件并写入字符
/*fn main() {
    let mut file = File::create("/Volumes/THU/lk2.txt").unwrap();
    file.write(b"FROM RUST PROGRAM").unwrap();
}*/
// 读取创建的文件的内容
fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/Volumes/THU/lk2.txt")?;

    file.write(b"COVER")?;

    Ok(())
}
