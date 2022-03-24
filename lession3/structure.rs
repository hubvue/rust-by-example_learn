// 结构体有3种类型，使用struct关键字类创建；
// - 元组结构体：事实上就是具名元组而已。
// - C风格结构体：也就是对象结构体
// - 单元结构体：不带字段，在泛型中很有用。

#[derive(Debug)]
struct Person {
  name: String,
  age: u8
}

// 单元结构体
#[derive(Debug)]
struct Unit;

// 元组结构体
#[derive(Debug)]
struct Pair(i32, i32);

// 带有两个字段的结构体
#[derive(Debug)]
struct Point {
  x: f32,
  y: f32
}

// 结构体可以作为另一个结构体的字段
// #[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point
}

fn rect_area(rect: Rectangle) -> f32 {
  let width = rect.bottom_right.x - rect.top_left.x;
  let height = rect.bottom_right.y - rect.top_left.y;
  width * height
}

fn square(point: Point, w: f32) -> Rectangle {
  let bootom_rightt = Point {
    x: point.x + w,
    y: point.y + w
  };
  Rectangle {
    top_left: point,
    bottom_right: bootom_rightt
  }
}
fn main() {
  let name = String::from("kim");
  let age = 24;
  let person = Person { 
    name, 
    age
  };
  println!("{:?}", person);
  println!("{:?}", person.age);
  println!("{:?}", person.name);

  // 实例化结构体 Point
  let point = Point {
    x: 10.3,
    y: 0.4
  };
  println!("{:?}", point);
  println!("{}, {}", point.x, point.y);

  // 使用结构体更新语法创建新的point，更新语法也就是扩展运算符，注意为..
  let bottom_right = Point {
    x: 5.2,
    ..point
  };
  println!("{:?}", bottom_right);

  // 解构point
  let Point {x, y} = point;
  println!("{}, {}", x, y);
  // 重命名解构
  let Point {x: left_edge, y: top_edge } = point;
  println!("{}, {}", left_edge, top_edge); 
  
  let rectangle = Rectangle {
    // 结构体的实例化也是一个表达式
    top_left: Point {
      x: left_edge,
      y: top_edge
    },
    bottom_right: bottom_right
  };
  println!("rectangle: {:?}", rectangle);
  println!("{:?}, {:?}", rectangle.bottom_right, rectangle.top_left);
  println!("rect_area: {}", rect_area(rectangle));

  // 单元结构体
  let unit = Unit;
  println!("{:?}", unit);

  // 元组结构体
  let pair = Pair(1, 2);
  println!("{:?}, {}, {}", pair, pair.0, pair.1);

  // 解构元组结构体
  let Pair(integer, decimal) = pair;
  println!("{}, {}", integer, decimal);

  println!("{:?}", square(point, 10.2));
}
