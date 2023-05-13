// hashmaps1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a hint.



use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    shove_into(&mut basket,"banana",2);
    shove_into(&mut basket,"mango",6);
    shove_into(&mut basket,"apple",4);
    shove_into(&mut basket,"pear",2);
    shove_into(&mut basket,"rhubarb",1);
    basket
}

fn shove_into (basket: &mut HashMap<String, u32>, fruit: &str, amount: u32) {
    basket.insert(String::from(fruit), amount);
}

fn main () {
    let basket = fruit_basket();
    for (fruit, amount) in &basket {
        println!("{}: {}", fruit, amount);
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
