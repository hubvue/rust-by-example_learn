// 可变参数接口
// 可变参数接口可以接受任意数目的参数。比如说println就可以，其参数的数目是由格式化字符串指定的

macro_rules! calculate {
    (eval $e: expr) => {{
      {
        let val: usize = $e;
        println!("{} = {}", stringify!($e), val);
      }
    }};
    (eval $e: expr, $(eval $es:expr),+) => {{
      calculate! { eval $e }
      calculate! { $(eval $es),+ }
    }}
}

fn main() {
    calculate! {
      eval 1 + 2,
      eval 3 + 4,
      eval (2 * 3) + 1
    }
}
