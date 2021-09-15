fn main() {
let x = " lei kang".to_string();
let y = " java learn ".to_string();
let _y_x = format!("{} {}",x,y);
let _y_x = format!("{0} {1}",x,y);

    println!("_y_x is {}",_y_x);
}
