// DSL 领域特定语言
//  DSL 是Rust的宏中集成的微型语言。这种语言是完全合法的，因为宏系统会把它转换成普通的Rust语法树，它只不过看起来像是另一种语言而已。
// 这就允许你为一些特定功能创造一套简洁直观的语法（当然是有限制的）

// 比如说我想要定义一套小的计算器API，可以传给它表达式，它会把结果打印到控制台上。

macro_rules! calculate {
    (eval $e: expr) => {{
      {
        let val: usize = $e;
        println!("{} = {}", stringify!{$e}, val);
      }
    }};
}

fn main() {
    calculate! {
      eval 1 + 2
    }
    calculate! {
      eval (1 + 2) * (3 / 4)
    }
}
