// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        // naive:
        // if optional_target.is_some() {
        //     let word = optional_target.unwrap();
        //     assert_eq!(word, target);
        // }
        // naive 2:
        // if let word = optional_target.unwrap() {
        //     assert_eq!(word, target);
        // }
        // actual:
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }

    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let
        // naive:
        // while range>0 {
        //         if let integer = optional_integers.pop().unwrap().unwrap() {
        //         assert_eq!(integer, range);
        //         range -= 1;
        //     }
        // }
        // actual:
        while let Some(vector_element) = optional_integers.pop() { // optional_integers.pop() returns Option<Option<i8>>, so we unwrap it by using while let
            if let Some(integer) = vector_element { // integer is of type i8 now
                assert_eq!(integer, range);
            }
            range -= 1;
        }
    }
}
