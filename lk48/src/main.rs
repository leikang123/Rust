fn main() {

    println!("{:?}",learn_lang("rust"));
    println!("{:?}",learn_lang("java"));
}
// 定义一个函数接受一个参数my_leaern,该参数引用str，并返回一个Option<bool>
fn learn_lang(my_learn:&str) -> Option <bool>{
    if my_learn == "rust"{
        // 返回值的类型是bool
        Some(true)
    }else{
        // 如果没有的话返回无，用None表示
        None
    }

}
