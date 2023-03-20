use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::id::Id;

use super::tagable::TagableId;

pub struct TagID;
pub type TagId = Id<TagID>;
#[derive(Serialize, Deserialize)]
pub struct Tag {
    id: TagId,
    title: String,
    parent: Option<TagId>,
    references: HashSet<TagableId>,
}

impl Tag {
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

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub(super) fn add_ref(&mut self, tagable_id: TagableId) {
        self.references.insert(tagable_id);
    }

    pub(super) fn rm_ref(&mut self, tagable_id: &TagableId) {
        self.references.remove(tagable_id);
    }

    
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

impl Eq for Tag {}

