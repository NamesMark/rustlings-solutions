// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.



use std::num::ParseIntError;

pub fn very_parse_much_wow (value: &str) -> Result<i32, ParseIntError> {
    value.parse::<i32>()
}

#[cfg(test)]
mod tests {
    use crate::very_parse_much_wow;
    #[test]
    fn you_can_assert_eq() {
        let element_one = very_parse_much_wow("69420").unwrap();
        assert_eq!(element_one,69420);
    }
}
