// traits3.rs
//
// Your task is to implement the Licensed trait for
// both structures and have them return the same
// information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a hint.



pub trait Licensed {
    fn licensing_info(&self) -> String {
        //String::from(self.version_number) // impossible without editing the 2 impls below
        String::from("Some information") // lmao this exercise is kinda weird
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

// tried, but it still requires licensing_info for SomeSoftware and OtherSoftware:
// enum AnySoftware {
//     OneSoft(SomeSoftware),
//     SecondSoft(OtherSoftware)
// }

// impl Licensed for AnySoftware {
//     fn licensing_info(&self) -> String {
//         match self {
//             AnySoftware::OneSoft(one_software) => one_software.licensing_info(),
//             AnySoftware::SecondSoft(another_software) => another_software.licensing_info()
//         }
//     }
// }

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
