// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.


struct ColorClassicStruct {
    // TODO: Something goes here
    red:i32,
    green:i32,
    blue:i32,
}
// 定义元组结构体：相当于是值类型
struct ColorTupleStruct(i32,i32,i32);

#[derive(Debug)]
struct UnitLikeStruct;
// 没有任何字段的结构：类单元结构体。 
// 因为和元组类型一样，用途为 只需要实现某些trait，但不要类中存储数据

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct{
            red:0,
            green:255,
            blue:0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(0,255,0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
