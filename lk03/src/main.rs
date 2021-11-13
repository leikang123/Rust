// 定义函数reser(),带有参数mut arr:[] 定义可写数组arr,arr数组类型为u32,5个元素
fn reser(mut arr:[u32;5]){

    arr [0] =1;
    arr[1] = 2;
    arr[2] = 3;
    arr[3] =4;
    arr[4] = 5;
}
fn main(){
    let arr:[u32;5]= [1,2,3,4,5];
    reser(arr);
    print!("is {:?}",arr);
}




