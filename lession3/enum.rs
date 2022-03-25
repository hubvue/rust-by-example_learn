// enum 关键字允许创建一个从数个不同取值中选其一的枚举类型。
// 任何一个在struct中合法的取值在enum中也合法。

// 该属性用于隐藏对未使用代码的警告
#[allow(dead_code)]
enum WebEvent {
  PageLoad,  //单元结构体
  PageUnload,
  KeyPress(char),  // 元组结构体
  Paste(String),
  Click{x: i64, y: i64} // 普通结构体
}

// enum 类型别名
type Event = WebEvent;

fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad => println!("page loaded"),
    WebEvent::PageUnload => println!("page unloaded"),
    WebEvent::KeyPress(c) => println!("keypress: {}", c),
    WebEvent::Paste(s) => println!("pasted: {}", s),
    WebEvent::Click{x, y} => {
      println!("click at x = {}, y = {}", x, y);
    }
  }
}

// 在 impl块中使用Self别名
enum VeryVerboseEnumOfThingsToDoWidthNumbers {
  Add,
  Subtract
}

impl VeryVerboseEnumOfThingsToDoWidthNumbers {
  fn run(&self, x: i32, y: i32) -> i32 {
    match self {
      Self::Add => x + y,
      Self::Subtract => x - y,
    }
  }
}

fn main() {
  println!("enum");

  let pressed = WebEvent::KeyPress('x');
  // to_owned() 从一个字符串切片中创建一个具有所有权的 String
  let pasted = WebEvent::Paste("my test".to_owned());
  let click = WebEvent::Click {
  x: 20,
    y: 80
  };
  let load = WebEvent::PageLoad;
  let unload = WebEvent::PageUnload;

  
  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);

  let pressed_by_event = Event::KeyPress('y');
  inspect(pressed_by_event);

  let add = VeryVerboseEnumOfThingsToDoWidthNumbers::Add;
  let subtract = VeryVerboseEnumOfThingsToDoWidthNumbers::Subtract;

  println!("{}", add.run(1, 2));
  println!("{}", subtract.run(3, 2));
}
