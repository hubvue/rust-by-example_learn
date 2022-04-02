// 方法是依附于对象的函数，这些方法通过关键字self来访问对象中的数据和其他。
// 方法在impl代码块中定义。

struct Point {
  x: f64,
  y: f64
}

// 实现的代码块， Point的所有方法都在这里给出
impl Point {
  // 这是一个静态方法（static method）
  // 静态方法不需要被实例调用
  // 这类方法一般用作构造器
  fn origin() -> Point {
    Point{ x: 0.0, y: 0.0 }
  }
  fn new (x: f64, y: f64) -> Point {
    Point {
      x: x,
      y: y
    }
  }
}

struct Rectangle {
  p1: Point,
  p2: Point
}

impl Rectangle {
  // 这是一个实例方法，带有self注入的是实例方法，不带的是静态方法
  // &self是self: &Self的语法糖，Self是类比与this
  fn area(&self) -> f64 {
    let Point {x: x1, y: y1} = self.p1;
    let Point {x: x2, y: y2} = self.p2;
    ((x1 - x2) * (y1 - y2)).abs() // abs是f64上的一个方法，犯规调用者得的绝对值
  }

  fn perimeter(&self) -> f64 {
    let Point {x: x1, y: y1} = self.p1;
    let Point {x: x2, y: y2} = self.p2;

    2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
  }

  // &mut self 是 self: &mut Self 的语法糖
  fn translate(&mut self, x: f64, y: f64) {
    self.p1.x += x;
    self.p2.x += x;
    
    self.p1.y += y;
    self.p2.y += y;
  }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
  // 这个方法会消耗调用者的资源
  fn destory(self) {
    let Pair(first, second) = self;
    println!("Destorying Pair({}, {})", first, second);
  }
}

fn main() {
  let rectangle = Rectangle {
    // 静态方法使用::调用
    p1: Point::origin(),
    p2: Point::new(3.0, 4.0)
  };

  println!(" perimeter: {}", rectangle.perimeter());
  println!(" area: {}", rectangle.area());

  let mut square = Rectangle {
    p1: Point::new(1.0, 1.0),
    p2: Point::origin()
  };

  println!("{}", square.area());
  // 可变方法调用可变对象
  square.translate(1.0, 1.0);
  println!("{}", square.p1.x);

  let pair = Pair(Box::new(1), Box::new(2));

  pair.destory();
  pair.destory();

}
