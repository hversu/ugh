use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: Option<i32>,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    pub source: String,
    pub target: String,
    pub relation_type: String,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    pub id: Option<i32>,
    #[serde(flatten)]
    pub other: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Properties {
    pub fn new() -> Self {
        Properties {
            id: None,
            other: HashMap::new(),
        }
    }

    pub fn map_values(props: HashMap<String, String>) -> Self {
        let mut id: Option<i32> = None;
        let mut other = HashMap::new();
        for (key, value) in props {
            if key == "id" {
                id = Properties::get_id_from_str(&value);
            } else {
                other.insert(key, value);
            }
        }
        Properties {
            id,
            other,
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.other.get(key)
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.other.insert(key, value);
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.other.remove(key)
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub fn get_id(&self) -> Option<i32> {
        self.id
    }

    pub fn set_id_from_str(&mut self, id: &str) {
        self.id = Some(id.parse().unwrap());
    }

    pub fn get_id_as_str(&self) -> Option<String> {
        self.id.map(|id| id.to_string())
    }

    pub fn get_id_from_str(id: &str) -> Option<i32> {
        let mut new_id = id.replace("n", "");
        new_id = new_id.replace("e", "");
        match new_id.parse() {
            Ok(id) => Some(id),
            Err(_) => None,
        }
    }

}

// Implementing IntoIterator for Properties
impl IntoIterator for Properties {
    type Item = (String, String);
    type IntoIter = std::collections::hash_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.other.into_iter()
    }
}

// Implementing IntoIterator for &Properties
impl<'a> IntoIterator for &'a Properties {
    type Item = (&'a String, &'a String);
    type IntoIter = std::collections::hash_map::Iter<'a, String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.other.iter()
    }
}
