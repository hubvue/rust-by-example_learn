// process::Output 结构体表示已结束的子进程的输出，而process::Command
// 结构体时一个进程创建者。

use std::process::Command;

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
