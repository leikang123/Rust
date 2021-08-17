mod a;
mod b;
mod c;

// 函数中定义范型&【i32】
fn max(array: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

fn main() {
    let a = [2, 4, 6, 3, 1];
    println!("max = {}", max(&a));
}
