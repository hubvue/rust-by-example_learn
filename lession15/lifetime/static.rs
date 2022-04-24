// static
// 'static生命周期是可能的生命周期中最长的，它会在整个程序运行的时期中存在。
// 'static生命周期也可被强制转换成一个更短的生命周期。
//  有两种方式使变量拥有'static生命周期，它们都把数据保存在可执行文件的只读内存区：
// - 使用static 声明来创建常量；
// - 创建一个拥有 &'static str类型的string字面量

static NUM: i32 = 18;

// 返回一个指向NUM的引用，该引用不取NUM的static生命周期，
// 而是被强制转换和输入参数一样的生命周期
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        let static_string = "I am is read-only memory";
        println!("static_string: {}", static_string);
        // 当static_string 离开作用域时，该引用不能再使用，不过数据任然存在于二进制文件里面。
    }
    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {}", NUM);
}
