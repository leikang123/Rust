fn main() {
let check = my_all(6);
assert_eq!(check.is_ok(),true);
let check2 = my_all(2);
assert_eq!(check2.is_err(),true);
}
fn my_all(x:i32) -> Result<String,String> {
    if x % 3 == 0{
        Ok("java is good".to_string())
    }else {
        Err("Not is ".to_string())
    }
}