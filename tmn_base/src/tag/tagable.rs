use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::id::Id;
use super::tag::{Tag, TagId};

pub struct TagableID;
pub type TagableId = Id<TagableID>;

#[derive(Serialize, Deserialize)]
pub struct Tags {
    tagable_id: TagableId, 
    tags: HashSet<TagId>,
}

impl Tags {
    pub fn new(id: TagableId) -> Self {
        Tags {
            tagable_id: id,
            tags: HashSet::new(),
        }
    }
}

pub trait Tagable {
    fn get_mut_tags(&mut self) -> &mut Tags;
    fn get_tags(&self) -> &Tags;
    
    fn get_tagable_id(&self) -> &TagableId {
        &self.get_tags().tagable_id
    }
    
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
            tag.rm_ref(&self.get_tagable_id());
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
