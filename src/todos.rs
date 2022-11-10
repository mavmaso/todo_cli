use std::{collections::HashMap, io::Read};

pub struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    pub fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    pub fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)   
        }

        std::fs::write("db.txt", content)
    }

    pub fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

        let mut content = String::new();

        f.read_to_string(&mut content)?;

        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), v.parse().unwrap()))
            .collect();

        Ok(Todo { map })
    }

    pub fn complete(&mut self, item: &String) -> Option<()> {
        match self.map.get_mut(item) {
            Some(v) => Some(*v = false),
            None => None
        }
    }
}