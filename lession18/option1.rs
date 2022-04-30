// 使用？解开Option

//可以使用match语句来解开Option，但使用？运算符通常会更容易。
// 如果x是Option，那么若x是Some，对x?表达式求职将返回低层值，否则无论函数是否正在执行都将终止且返回None。

// fn next_birthday(current_age: Option<u8>) -> Option<String> {
//     // 如果 current_age 是None，则将返回None
//     // 否则将Some内部的u8赋值给next_age
//     let next_age: u8 = current_age?;
//     Some(format!("Next year I will be{}", next_age))
// }

// 可以将多个 ? 链接再一起，以使代码更具有可读性。
struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // 获取此人的工作电话号码的区号
    fn work_phone_area_code(&self) -> Option<u8> {
        // 使用?的简单写法
        self.job?.phone_number?.area_code
        // 使用unwrap的写法
        // self.job.unwrap().phone_number.unwrap().area_code
        // 使用match 的写法
        // match self.job {
        //     Some(job) => match job.phone_number {
        //         Some(phone) => phone.area_code,
        //         None => None,
        //     },
        //     None => None,
        // }
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 32822222,
            }),
        }),
    };
    println!("{:?}", p.work_phone_area_code());
}
