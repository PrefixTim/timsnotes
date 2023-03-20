use serde::{Deserialize, Serialize};

use crate::{
    id::Id,
    tag::{Tagable, TagableId, Tags},
};

pub struct ThoughtID;
pub type ThoughtId = Id<ThoughtID>;

#[derive(Serialize, Deserialize)]
pub struct Thought {
    id: ThoughtId,
    title: String,
    tags: Tags,
    content: String,
}

impl Thought {
    fn new(id: ThoughtId, title: String) -> Self {
        Thought {
            id: id.clone().into(),
            title: title,
            tags: Tags::new(id.into()),
            content: "".to_owned(),
        }
    }

    fn add_content(&mut self, content: String) {
        self.content = content;
    }
}

impl From<ThoughtId> for TagableId {
    fn from(value: ThoughtId) -> Self {
        value.id.into()
    }
}

impl Tagable for Thought {
    fn get_mut_tags(&mut self) -> &mut Tags {
        &mut self.tags
    }

    fn get_tags(&self) -> &Tags {
        &self.tags
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use uuid::Uuid;

    use super::*;
    use crate::tag::Tag;

    #[test]
    fn test_content() {
        let ids = vec![
            Uuid::from_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap(),
            Uuid::from_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d9").unwrap(),
        ];
        let mut thoughts = vec![
            Thought::new(ids[0].into(), "Th1".to_owned()),
            Thought::new(ids[1].into(), "Th2".to_owned()),
        ];

        thoughts[0].add_content("content".to_owned());
        thoughts[1].add_content("content asdf".to_owned());

        assert!(thoughts[0].content != thoughts[1].content)
    }

    #[test]
    fn test_tags_in_thoughts() {
        let ids = vec![
            Uuid::from_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap(),
            Uuid::from_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d9").unwrap(),
            Uuid::from_str("a2a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap(),
            Uuid::from_str("a2a2a3a4b1b2c1c2d1d2d3d4d5d6d7d9").unwrap(),
        ];

        let mut tags = vec![
            Tag::new(ids[0].into(), "T1".to_owned()),
            Tag::new(ids[1].into(), "T2".to_owned()),
        ];
        let mut thoughts = vec![
            Thought::new(ids[2].into(), "Th1".to_owned()),
            Thought::new(ids[2].into(), "Th2".to_owned()),
        ];

        assert!(thoughts[0].tag(&mut tags[0]).is_ok());
        assert!(thoughts[0].tag(&mut tags[1]).is_ok());
        assert!(thoughts[1].tag(&mut tags[1]).is_ok());

        assert!(thoughts[0].has_tag(&tags[0]));
        assert!(thoughts[0].has_tag(&tags[1]));
        assert!(thoughts[1].has_tag(&tags[1]));
        assert!(!thoughts[1].has_tag(&tags[0]));

        assert!(thoughts[1].untag(&mut tags[0]).is_err());

        assert!(thoughts[0].untag(&mut tags[1]).is_ok());

        assert!(thoughts[0].has_tag(&tags[0]));
        assert!(!thoughts[0].has_tag(&tags[1]));
        assert!(thoughts[1].has_tag(&tags[1]));
        assert!(!thoughts[1].has_tag(&tags[0]));

        assert!(thoughts[1].untag(&mut tags[1]).is_ok());

        assert!(thoughts[0].has_tag(&tags[0]));
        assert!(!thoughts[0].has_tag(&tags[1]));
        assert!(!thoughts[1].has_tag(&tags[1]));
        assert!(!thoughts[1].has_tag(&tags[0]));

        assert!(thoughts[0].untag(&mut tags[0]).is_ok());

        assert!(!thoughts[0].has_tag(&tags[0]));
        assert!(!thoughts[0].has_tag(&tags[1]));
        assert!(!thoughts[1].has_tag(&tags[1]));
        assert!(!thoughts[1].has_tag(&tags[0]));
    }
}
