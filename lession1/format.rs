// 格式化输出操作由 std::fmt 里面所定义的一系列宏来处理，包括：
// 1. format！：将格式化文本写到字符串。
// 2. print!：与format!类似，但将文本输出到控制台(io::stdout)。
// 3. println!：与print!类似，但输出结果追加一个换行符。
// 4. eprint!：与print!类似，但将本文输出到标准错误(io::stderr)
// 5. eprintln!：与eprint!类似，但输出结果追加一个换行符。

// 这些宏都以相同的做法(模板字符串)解析本文。有个额外优点是格式化的正确性会在编译时检查

fn main() {
  // 模板字符串中的{}会被后续参数替代，变量内容会转化为字符串
  println!("{} days", 31);

  // 变量可以通过加后缀来改变变量的类型, 数字默认i32类型
  println!("{} days", 31f32);

  // 模板字符串可以通过位置参数填充
  println!("{0}, this is {1}, {1} this is {0}", "Alice", "Bob");

  // 模板字符串可以通过命名参数填充
  println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

  // 模块字符串指定特殊格式的输出{命名:格式}
  println!("{} of {}, {name:b} people know binary, the other half do not", 1, 2, name=10);

  // 指定宽度来右对齐文本
  println!("{number:>width$}", number=1,width=6);
  // 指定宽度的同时设置空格补充
  println!("{number:>0width$}", number=1, width=6);
  println!("{string:>width$}", string="abc", width=10);

  // 指定小数位数的输出
  println!("{:.3}", 3.1415926)
}

// fmt::Debug ：使用 {:?} 标记，，格式化文本以供调试使用。
// fmt::Display ：使用 {} 标记。以更优化和友好的风格来格式化文本
