use serde::{Serialize,Deserialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: Uuid,
    title: String,
}

impl Tag {
    pub fn new(id: Uuid, title: String) -> Tag {
        Tag { id, title }
    }
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
