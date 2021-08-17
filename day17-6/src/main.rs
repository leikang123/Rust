// use std::intrinsics::prefetch_read_instruction;

fn main() {
    /*   // 创建一个空数组
    let mut vector = vec![];
    // 添加数组元素
    vector.push(12);
    vector.push(23);
    vector.push(27);
    vector.push(68);
    vector.push(87);
    vector.push(32);
    vector.push(600);
    // 输出数组
    println!("is {:?}", vector);*/
    // 定义两个数组xy,语法格式: let mut 变量名: Vec<类型> =vec![];
    let mut x = vec![1, 4, 9];
    let mut y = vec![23, 56, 78];
    x.append(&mut y);
    println!("{:?}", z);
    // print!("{}", z[2])
}
