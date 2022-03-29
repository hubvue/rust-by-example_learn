// 用enum实现链表

use List::*;

// Box<T>：Box <T> 是一个智能指针，指向在类型为T的堆上分配的数据。 Box <T> 允许将数据存储在堆而不是堆栈上。 
//         Box <T> 是一个拥有的指针。 除了将数据存储在堆上之外， Box 没有性能开销。
#[derive(Debug)]
enum List {
  // Cons：元组结构体，包含链表的第一个元素和一个指向下一个节点的指针
  Cons(u32, Box<List>), 
  // 未节点，表明链表结束
  Nil
}

// 定义和实现enum上的方法
impl List {
  // 创建一个空的List实例
  fn new() -> List {
    Nil // List::Nil
  }

  // 处理一个List，在其头部插入新元素，并返回该List
  fn prepend(self, elem: u32) -> List {
    Cons(elem, Box::new(self))
  }
  // 返回List的长度
  fn len(&self) -> u32 {
    // 必须对 self进行match，因为这个方法的行为取决于self的取值种类
    // self为&List类型，*self为List类型
    match *self {
      // 不能得到tail的使用权，因为self是借用的
      // 因此使用一个对tail的引用
      Cons(_, ref tail) => 1 + tail.len(),
      // 递归
      Nil => 0
    }
  }

  // 返回列表的字符串表示（该字符串是堆分配的）
  fn stringify(&self) -> String {
    match *self {
      Cons(head, ref tail) => {
        format!("{}, {}", head, tail.stringify())
      },
      Nil => {
        format!("Nil")
      }
    }
  }
}

fn main() {
  let mut list = List::new();

  list = list.prepend(1);
  list = list.prepend(2);
  list = list.prepend(3);
  println!("{:?}", list);

  println!("len: {}", list.len());
  println!("stringify: {}", list.stringify());
}
