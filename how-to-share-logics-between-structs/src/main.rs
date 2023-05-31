mod my {
    use thiserror::Error;

    pub fn parse<T>(prefix: &str, id: &str, new: fn(id: String) -> T) -> Result<T, InvalidId> {
        if id.starts_with(prefix) {
            Ok(new(id.to_string()))
        } else {
            Err(InvalidId(id.to_string()))
        }
    }

    pub fn generate<T>(prefix: &str, new: fn(id: String) -> T) -> T {
        new(prefix.to_owned() + &random_string::generate(12, "1234567890"))
    }
    #[derive(Error, Debug)]
    #[error("Invalid id")]
    pub struct InvalidId(pub String);

    #[macro_export]
    macro_rules! prefix {
        ($prefix:literal, $new:expr) => {
            pub fn parse(id: &str) -> Result<Self, $crate::my::InvalidId> {
                $crate::my::parse($prefix, id, $new)
            }

            pub fn generate() -> Self {
                $crate::my::generate($prefix, $new)
            }
        };
    }
}

pub mod domain {
    use crate::prefix;

    #[derive(Debug)]
    pub struct TicketId(String);

    impl TicketId {
        prefix!("T", Self);

        pub fn value(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug)]
    pub struct ProjectId {
        value: String,
    }

    impl ProjectId {
        prefix!("PRJ", |value| Self { value });

        pub fn value(&self) -> &str {
            &self.value
        }
    }
}

#[cfg(test)]
mod client_code {
    use crate::domain::TicketId;

    #[test]
    fn main() {
        let ticket_id = TicketId::generate();
        let ticket_id = TicketId::parse(ticket_id.value()).unwrap();
        dbg!(ticket_id);
    }
}

// fn main() {
//     println!("Hello, world!");
// }
//
// mod id {
//     #![allow(dead_code)]
//     use thiserror::Error;
//
//     #[derive(Error, Debug)]
//     #[error("Invalid id")]
//     pub struct InvalidId(pub String);
//
//     pub struct Prefix<T>(String, fn(String) -> T);
//
//     impl<T> Prefix<T> {
//         pub fn new(prefix: &str, new: fn(String) -> T) -> Self {
//             Self(prefix.to_string(), new)
//         }
//     }
//
//     pub trait PrefixedId {
//         fn prefix() -> Prefix<Self>
//         where
//             Self: Sized;
//
//         fn generate() -> Self
//         where
//             Self: Sized,
//         {
//             let Prefix(prefix, new) = Self::prefix();
//             new(format!(
//                 "{}{}",
//                 prefix,
//                 random_string::generate(8, "1234567890")
//             ))
//         }
//
//         fn parse(id: String) -> Result<Self, InvalidId>
//         where
//             Self: Sized,
//         {
//             let Prefix(prefix, new) = Self::prefix();
//             if id.starts_with(&prefix) {
//                 Ok(new(id))
//             } else {
//                 Err(InvalidId(id))
//             }
//         }
//     }
// }
//
// #[cfg(test)]
// mod usage {
//     pub mod project {
//         use crate::id::{Prefix, PrefixedId};
//         use std::ops::Deref;
//
//         pub struct ProjectId(String);
//
//         impl PrefixedId for ProjectId {
//             fn prefix() -> Prefix<Self> {
//                 Prefix::new("pj", Self)
//             }
//         }
//
//         impl Deref for ProjectId {
//             type Target = String;
//             fn deref(&self) -> &Self::Target {
//                 &self.0
//             }
//         }
//     }
//
//     mod client_code {
//         use super::project::ProjectId;
//         use crate::id::PrefixedId;
//
//         #[test]
//         fn main() {
//             let id = ProjectId::generate();
//             println!("id: {}", id.to_owned());
//             let id = ProjectId::parse("pj12345678".to_string()).unwrap();
//             println!("id: {}", id.to_owned());
//             // let Prefix(prefix, new) = ProjectId::prefix();
//             //     ^^^^^^ cannot match against a tuple struct which contains private fields
//         }
//     }
// }
fn main() {}
