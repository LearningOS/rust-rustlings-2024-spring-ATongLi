// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // 获取引用权，引用权相当于是传指针
    // 进行操作，但没有所有权
    // &表示为引用,函数参数接收引用，也可在函数内部使用，而不获取所有权（借用）
    // 借用：将创建一个引用的行为称为借用（东西借过来用一下，但不据为己有）
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
