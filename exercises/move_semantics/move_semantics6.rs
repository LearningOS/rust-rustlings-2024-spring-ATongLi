// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    // 此处所有权不能被转移
    // 所以通过clone进行深拷贝赋值
    get_char(data.clone());

    string_uppercase(&data);

    println!("{}",data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}


// Should take ownership
fn string_uppercase(mut data: &String) {
   // 方法1: 引用，进行操作后，进行了重新赋值data
   // 这里的data就不是原来的引用data，而是新的data
   // let data = &data.to_uppercase();

   // 方法2：直接操作，直接读。
   data.to_uppercase();

   // 方法3: 不能同时存在读，写两种引用
    // let mut data = &data.to_uppercase();
    println!("{}", data);
}
