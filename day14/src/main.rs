fn main() {
    #[derive(Copy,Clone,Debug,PartialEq)]
    // 枚举类
    enum Book {
        // 种类{ 属性}
        // 纸质书的索引
        Papery { index: u32 },
        // 电子书的索引
        Electronic { url: String },
    }
    // 定义变量获取钟种类书的实力
    let book = Book::Papery { index: 1001 };
    let ebook = Book::Electronic {
        url: String::from("url..."),
    };
    // 实现枚举类，book为枚举类类型 也就是变量book/ebook
    match book {
        // 分之-》返回表达式
        Book::Papery { index } => {
            println!("Papery book {}", index);
        }
        // 分之-》返回表达式
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
}
