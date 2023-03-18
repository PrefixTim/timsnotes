use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use super::tagable::TagableId;

pub use i32 as TagId;

#[derive(Serialize, Deserialize)]
pub struct Tag {
    id: TagId,
    title: String,
    parent: Option<TagId>,
    references: HashSet<TagableId>,
}

impl Tag {
    // ToDo make tags unique
    pub fn new(id: TagId, title: String) -> Tag {
        Tag {
            id: id,
            title: title,
            parent: Option::None,
            references: HashSet::new(),
        }
    }

    pub fn get_id(&self) -> &TagId{
        &self.id
    }

    pub fn add_ref(&mut self, tagable_id: TagableId) {
        self.references.insert(tagable_id);
    }

    pub fn rm_ref(&mut self, tagable_id: &TagableId) {
        self.references.remove(tagable_id);
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

impl Eq for Tag {}
