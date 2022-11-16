use std::{collections::HashMap};

pub struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    pub fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    pub fn save(self) -> Result<(), std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        
        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(())
    }

    pub fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {map: HashMap::new()}),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }

    pub fn complete(&mut self, item: &String) -> Option<()> {
        match self.map.get_mut(item) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
