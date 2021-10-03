// 定义结构体
struct HouseRoo{
    houseSize:i64,
    hourseName:String,
    hourseColor:String,
}

fn main() {
    // 创建结构体实力
    let mut houseRoo_1 = HouseRoo{
        houseSize:12,
        hourseName:String::from("dog"),
        hourseColor:String::from("red"),
    };
    // 创建结构体实力
    let mut houseRoo_2 = HouseRoo{
        houseSize:21,
        hourseName:String::from("padgir"),
        hourseColor:String::from("blaue")
    };

    println!("houseSize {},hourseName {},hourseColor {}",houseRoo_1.houseSize,houseRoo_1.hourseName,houseRoo_1.hourseColor);
    println!("hourseSize {},hourseName {},hourseColor {}",houseRoo_2.houseSize,houseRoo_2.hourseName,houseRoo_2.hourseColor);
    houseRoo_1.hourseName = "jund".to_string();
    houseRoo_2.hourseColor = "write".to_string();
    println!("houseSize {},hourseName {},hourseColor {}",houseRoo_1.houseSize,houseRoo_1.hourseName,houseRoo_2.hourseColor);
}
