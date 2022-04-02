// 闭包本质上很灵活，能做功能要求的事情，使闭包在没有类型标注的情况下运行。
// 这使得捕获能够灵活的适应实例。即可 移动(move) 有可 借用 (borrow)。
// 闭包通过一下方式捕获变量：
// 1.通过引用：&T
// 2.通过可变引用：&mut T
// 3.通过值： T

// 闭包优先通过引用来捕获变量，并且仅在需要时使用其他方式。

use std::mem;

fn main() {
  let color = String::from("green");

  // 这个闭包打印 color， 它会立即借用(通过引用 &) color, 并且将该借用和闭包本身存储到print变量中。
  // color 会一直保持被借用状态直到 print 离开作用域
  let print = || println!("`color`: {}", color);

  // 使用借用来调用闭包 color
  print();
  print();

  // color 可 再次被 不可变 借用，因为闭包只持有一个指向 color 的不可变引用。
  let reborrow = &color;
  // let _color_moved = color;
  print();
  println!("{}", reborrow);

  // 在最后使用 print之后，移动或重新借用都是允许的
  // 当在借用过程中，不可移动
  let _color_moved = color;

  // println!("reborrow: {}", reborrow);

  let mut count = 0;
  // 这个闭包使 count 值增加。要做到这点，它需要得到 &mut count 或者 count 本身，单 &mut count 的要求没那么严格 ，所以采用这种方法。
  // 该闭包立即借用count
  // inc 前面需要加上mut，因为闭包里存储这一个&ut变量，调用闭包时，该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
  let mut inc = || {
    count += 1;
    println!("count: {}", count);
  };

  inc();
  // let _reborrow = count;
  inc();

  let _reborrow = &count;

  // 不可复制类型(non-copy type)
  let movable = Box::new(3);

  // mem::drap 要求 T类型本身，所以闭包将会捕获变量的值。这种情况下，可复制类型将会复制给闭包，而原始值不受影响。
  // 不可复制类型必须移动到闭包中，因而 movable 变量在这里立即移动到了闭包中。
  let consume = || {
    println!("movable {:?}", movable);
    mem::drop(movable);
  };
  // 变量已经发生移动
  // println!("{:?}"， movable);
  consume();
  // movable 已经被销毁
  // consume();

  // 在竖线 | 之前使用move会强制闭包取得被捕获变量的所有权
  // 加上move以移动的方式捕获变量的所有权
  // 去掉move以不可变借用的方式捕获变量

  let haystack = vec![1, 2, 3];

  
  let contains = move |needle| haystack.contains(needle);

  println!("{}", contains(&1));
  println!("{}", contains(&4));

  // println!("{:?}", haystack); // haystack 发生移动
}
