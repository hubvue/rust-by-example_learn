// 使用库
// 要讲一个crate链接到新建的库，可以使用rustc的--extern 选项。然后将所有的物件导入到与库名相同的模块下。此模块的操作通常与任何其他模块相同。
fn main() {
  lib::public_function();

  lib::indirect_access();
}

// liblib.rlib是已经编译好的库的路径
