fn main() {
    // define a vector of size 5   
    let my_vec = vec![1, 2, 3, 4, 5];
    // using loop
    let mut index = 0;
    for i in my_vec.iter(){ // it works even if .iter() is not written
        println!("Element at index {}:{} ", index, i);
        index = index + 1;
    }
}
