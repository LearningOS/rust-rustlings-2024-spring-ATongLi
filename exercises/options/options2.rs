// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let word = optional_target {
            assert_eq!(word, Some(target));
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        // Option可能里面面是没有值的None，或是有值Some(index)
        // 所以要对Option做判断的时候，也需要Some(index)包含起来
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.

        while let integer = optional_integers.pop() {
                assert_eq!(integer, Some(Some(cursor)));
                cursor -= 1;
                if cursor == 0 {
                    break;
                }
        }

        assert_eq!(cursor, 0);
    }
}
