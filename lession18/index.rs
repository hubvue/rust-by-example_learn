// 错误处理

// 错误处理是处理可能发生的失败情况的过程。例如读取一个文件时 失败了，如果继续使用这个无效的输入，那显然是有问题的。
// 注意到并且显式地处理这种错误可以避免程序的其他部分产生潜在的问题。

// 在Rust中有多种错误处理的方式。
// - 显式的panic主要用于测试，以及处理不可恢复的错误。在原型开发中这很有用，比如用来测试还没有实现的函数，不过这时使用unimplemented更能表达意图。另外在测试中，panic是一种显式地失败的好方法。
// - Option类型时为了值是可选的、或者缺少值并不是错误的情况准备的。比如说寻找父目录时，/和C:这样的目录就没有父目录，这应当并不是一个错误。当处理OPtion时，unwrap可用于原型开发，也可以用于能够确定Option中一定有值的情形。
//    然而expect更有用，因为它允许你指定一条错误信息，以免万一还是出现了错误。
// - 当错误有可能发生，且应当由调用者处理时，使用Result。也可以unwrap然后使用expect，但是除了在测试或者原型开发中，请不要这样做。
