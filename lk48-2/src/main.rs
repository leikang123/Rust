fn main() {
    println!("{:?}",my_lk("leikang"));
    println!("{:?}",my_lk("kangshifu"));
    
}
fn my_lk(x_y:&str) -> Option<bool>{
    if x_y == "leikang"{
        Some(true)
    }else {
        None
    }
}

