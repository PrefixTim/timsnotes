use std::{hash::{Hash, Hasher}, marker::PhantomData};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Id<T> {
    pub id: uuid::Uuid,
    _phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            _phantom: PhantomData,
        }
    }
}

impl<T> From<Uuid> for Id<T> {
    fn from(value: Uuid) -> Self {
        Self { id: value, _phantom: PhantomData }
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl <T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), _phantom: PhantomData }
    }
}