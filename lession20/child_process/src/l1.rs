// process::Output 结构体表示已结束的子进程的输出，而process::Command
// 结构体时一个进程创建者。

use std::{
    io::prelude::*,
    process::{Command, Stdio},
};

#[allow(dead_code)]
pub fn run() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was: {}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustd failed and stderr was: {}", s);
    }
}

// std::Child结构体代表了一个正在运行的子进程，它暴露了stdin、
// stdout和stderr句柄，从而可以通过管道与所代表的进程交互

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";
pub fn run_pipe() {
    // 启动wc命令
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("could not spawn wc: {:?}", why),
        Ok(process) => process,
    };

    // 将字符串写入wc的stdin
    // stdin拥有Option<ChildStdin>类型
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("could not write to wc stdin: {:?}", why),
        Ok(_) => println!("sent pangram to wc"),
    }

    // println!("{:?}", process.stdin);

    // 因为 stdin在上面调用后就不再存活，调用unwrap会触发process.stdin的Move语义

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("could not read wc stdout: {:?}", why),
        Ok(_) => println!("wc responded with: {}", s),
    }
}
