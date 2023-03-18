use serde::{Serialize,Deserialize};
use tag::{Tags, Tagable};
use uuid::Uuid;


mod tag;
#[derive(Serialize, Deserialize)]
pub struct Thought {
    id: Uuid,
    title: String,
    tags: Tags,
    content: String,
}

impl Thought {
    fn new(title: String) -> Self {
        Thought { id: Uuid::new_v4(), title: title, tags: Tags::new(), content: "".to_owned() }
    }

    fn add_content(&mut self, content: String) {
        self.content = content;
    }
}

impl  Tagable for Thought  {
    fn get_tagable_id(&self) -> &Uuid {
        &self.id
    }

    fn get_mut_tags(&mut self) -> &mut Tags {
        &mut self.tags
    }

    fn get_tags(&self) -> &Tags {
        &self.tags
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tag::Tag;

    #[test]
    fn test_content() {
        let mut thoughts = vec![Thought::new("Th1".to_owned()),Thought::new("Th2".to_owned())];

        thoughts[0].add_content("content".to_owned());
        thoughts[1].add_content("content asdf".to_owned());

        assert!(thoughts[0].content != thoughts[1].content)

    }

    #[test]
    fn test_tags_in_thoughts() {
        let mut tags = vec![Tag::new(0, "T1".to_owned()), Tag::new(1, "T2".to_owned())];
        let mut thoughts = vec![Thought::new("Th1".to_owned()),Thought::new("Th2".to_owned())];

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
