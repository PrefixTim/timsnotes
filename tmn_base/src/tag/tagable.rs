use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use super::tag::{Tag, TagId};

pub use uuid::Uuid as TagableId;

#[derive(Serialize, Deserialize)]
pub struct Tags {
    tags: HashSet<TagId>,
}

impl Tags {
    pub fn new() -> Self {
        Tags {
            tags: HashSet::new(),
        }
    }
}

pub trait Tagable {
    fn get_tagable_id(&self) -> &TagableId;
    fn get_mut_tags(&mut self) -> &mut Tags;
    fn get_tags(&self) -> &Tags;

    fn has_tag_id(&self, tagid: &TagId) -> bool {
        self.get_tags().tags.contains(tagid)
    }

    fn has_tag(&self, tag: &Tag) -> bool {
        self.get_tags().tags.contains(tag.get_id())
    }

    fn untag(&mut self, tag: &mut Tag) -> Result<(), ()> {
        if !self.has_tag(tag) {
            Err(())
        } else {
            tag.rm_ref(self.get_tagable_id());
            self.get_mut_tags().tags.remove(tag.get_id());
            Ok(())
        }
    }

    fn tag(&mut self, tag: &mut Tag) -> Result<(), ()> {
        if self.has_tag(tag) {
            Err(())
        } else {
            tag.add_ref(self.get_tagable_id().clone());
            self.get_mut_tags().tags.insert(tag.get_id().clone());
            Ok(())
        }
    }
}
