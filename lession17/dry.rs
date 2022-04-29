// DRY 不写冲农夫代码
// 通过提取函数或测试集的公共部分，宏可以让你写出DRY的代码。

// 这里给出一个例子，对Vec<T>实现并测试了关于+=，*=和-= 等运算符

use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // tt (token tree， 标记树) 指示符表示运算符和标记
    ($a: ident, $b: ident, $func: ident, $op: tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}

// 实现add_assing, mul_assing, sub_assign 函数
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

use std::iter;
macro_rules! test {
    ($func: ident, $x: expr, $y: expr, $z: expr) => {
        for size in 0usize..10 {
            let mut x: Vec<_> = iter::repeat($x).take(size).collect();
            let y: Vec<_> = iter::repeat($y).take(size).collect();
            let z: Vec<_> = iter::repeat($z).take(size).collect();

            println!("before x: {:?}", x);
            println!("bofore y: {:?}", y);
            $func(&mut x, &y);
            println!("after x: {:?}", x);
            println!("after y: {:?}", y);
            println!("z: {:?}", z);
        }
    };
}

fn main() {
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}
