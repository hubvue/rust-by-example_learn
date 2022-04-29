// 指示符
// 宏的参数使用一个美元符号$作为前缀，并使用一个指示符来注明类型：

macro_rules! create_function {
    // ident指示符用于变量名或函数名
    ($func_name: ident) => {
        fn $func_name() {
            // stringify!宏把ident转换为字符串
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// 借助上述宏来创建名为 foo 和 bar的函数
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // 此宏接受一个expr类型的表达式，并将它作为字符串，连同其结果一起打印出来
    // expr 指示符表示表达式
    ($expression: expr) => {
        // stringify! 将表达式原样转换成一个字符串
        println!("{:?} = {:?}", stringify!($expression), $expression)
    };
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);
}

// 全部的指示符：
// - block
// - expr 用于表达式
// - ident 用于变量名或函数名
// - item
// - pat  模式
// - path
// - stmt 语句
// - tt  标记树
// - ty 类型
