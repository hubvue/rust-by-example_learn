// 文件操作

//  File所有的方法都返回io::Result<T>类型，Result<T, io::Error>的别名

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// 打开文件
//  open 静态方法能够以只读方式打开一个文件
//  File拥有资源，即文件描述符，它会在自身被drop时关闭文件。

fn main() {
    // 创建指向所需的文件的路径
    let path = Path::new("./Cargo.toml");
    let display = path.display();

    // 以只读方式打开路径，返回io::Result<File>
    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {:?}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("could not read {}: {:?}", display, why),
        Ok(_) => println!("{} contains: \n{}", display, s),
    }

    // file离开作用域，并且文件将被关闭
}
