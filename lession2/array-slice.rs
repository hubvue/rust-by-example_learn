// 数组是一组拥有相同类型T的对象的集合，在内存中时连续存储的。数组使用中括号[]来创建，且它们的大小在编译时会被确定。
// 数据的类型标记为[T; length]：T为元素类型，length为数组大小

// 切片类型和数组类似，但其大小在编译时是不确定的。
// 相反，切片是一个双字对象，第一个字是一个指向数据的指针，第二个字是切片的长度。
// 这个“字”的宽度和usize相同，由处理器架构决定，比如在x86-64平台上就是64位。
// slice可以用来借用数组的一部分。
// slice的类型标记为&[T]

use std::mem;

fn main() {
  // 定长数组（类型标记是多余的）
  let xs: [i32; 5] = [1, 2, 3, 4, 5];
  println!("{:?}", xs);

  // 所有元素可以初始化相同的值, [值;长度]
  let ys: [i32; 1000] = [10;1000];
  println!("{:?}", ys);

  // 使用数组下标访问元素
  println!("{}", xs[0]);
  println!("{}", ys[0]);

  // 使用len方法获取数组的长度
  println!("{}", xs.len());
  println!("{:?}", xs.last());
  println!("{}", ys.len());

  // 数组是在栈中分配的，栈上分配大小都会len() * 4 ?
  println!("{}", mem::size_of_val(&xs));
  println!("{}", mem::size_of_val(&ys));

  // 数组在被借用后会成为slice
  let slice_xs: &[i32] = &xs;
  println!("{:?}", slice_xs);
  // slice可以指向数组的一部分
  println!("{:?}", &ys[1 .. 10]);

  // 数组越界会引发panic
  println!("{}", xs[100]);

}
