fn main() {
  // 定义变量只读contdion,bool类型,用于判断false ,true;
let contdion = true;
// 定义变量number，他的值是conditionde的类型然后判断
// 如果contdion =true的话，就输出5，false的话就输出9
let number = if contdion {
  5
}else {
  9
}; 
    println!("The value number is {} ",number);
}
