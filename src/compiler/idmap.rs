use super::common::Id;
use std::collections::HashMap;

pub struct IdMap {
    ids: HashMap<String, Id>,
    id_count: Id,
}

impl IdMap {
    pub fn new() -> Self {
        IdMap {
            ids: HashMap::new(),
            id_count: 0,
        }
    }

    pub fn get(&self, key: &str) -> Id {
        *self.ids.get(key).unwrap()
    }

    pub fn insert(&mut self, key: String) -> Id {
        let id = self.id_count;
        self.ids.insert(key, id);
        self.id_count += 1;
        id
    }

    pub fn add(&mut self) -> Id {
        let id = self.id_count;
        self.id_count += 1;
        id
    }
}
