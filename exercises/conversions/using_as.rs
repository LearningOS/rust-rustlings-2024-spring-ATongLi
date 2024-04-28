// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.


// 类型转换在Rust中通过使用`as`运算符来完成
// as关键字用于将一个类型转换为另一个类型,并且as关键字只能用于转换相同的基本类型，
// 例如i32到u32，f32到f64等。如果要转换不同的类型，可以使用From和Into trait
// fn average(values: &[f64]) -> f64 {
//     let sum: f64 = values.iter().sum();
//     sum / values.len() as f64
// }

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
