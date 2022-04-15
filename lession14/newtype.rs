// new type 惯用法
// new type 惯用法能保证在编译时，提供给程序的都是正确的类型

// 比如：实现一个年龄认证的函数，它要求输入的类型必须是Years类型

struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("old enough: {}", old_enough(&age));
    println!("old enough: {}", old_enough(&age_days.to_years()));
    // println!("old enough: {}", old_enough(&age_days));
}
