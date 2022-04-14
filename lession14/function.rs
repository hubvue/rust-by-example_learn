// 调用泛型函数有时需要显式地指明类型参量。这可能是因为调用返回类型时泛型的函数，或者编译器没有足够的信息来推断类型函数。
// 调用函数时，使用显式指定的类型参数像是这样 fun::<A, B, C, ...>();

struct A;
struct S(A);
struct SGen<T>(T); //泛型

// 普通函数
fn reg_fn(_s: S) {}

// 参数泛型
fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

// 函数泛型
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    generic::<char>(SGen('a')); // 显式指定泛型类型

    generic(SGen('c')); //隐式指定泛型类型
}
