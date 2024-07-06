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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties_new() {
        let props = Properties::new();
        assert_eq!(props.id, None);
        assert_eq!(props.other.len(), 0);
    }

    #[test]
    fn test_properties_map_values() {
        let mut props = HashMap::new();
        props.insert("id".to_string(), "1".to_string());
        props.insert("label".to_string(), "Node 1".to_string());
        props.insert("type".to_string(), "unknown".to_string());
        let properties = Properties::map_values(props);
        assert_eq!(properties.id, Some(1));
        assert_eq!(properties.other.len(), 2);
    }

    #[test]
    fn test_properties_get() {
        let mut props = Properties::new();
        props.insert("label".to_string(), "Node 1".to_string());
        assert_eq!(props.get("label"), Some(&"Node 1".to_string()));
    }

    #[test]
    fn test_properties_insert() {
        let mut props = Properties::new();
        props.insert("label".to_string(), "Node 1".to_string());
        assert_eq!(props.other.len(), 1);
    }

    #[test]
    fn test_properties_remove() {
        let mut props = Properties::new();
        props.insert("label".to_string(), "Node 1".to_string());
        assert_eq!(props.remove("label"), Some("Node 1".to_string()));
        assert_eq!(props.other.len(), 0);
    }

    #[test]
    fn test_properties_set_id() {
        let mut props = Properties::new();
        props.set_id(1);
        assert_eq!(props.id, Some(1));
    }

    #[test]
    fn test_properties_get_id() {
        let mut props = Properties::new();
        props.set_id(1);
        assert_eq!(props.get_id(), Some(1));
    }

    #[test]
    fn test_properties_set_id_from_str() {
        let mut props = Properties::new();
        props.set_id_from_str("1");
        assert_eq!(props.id, Some(1));
    }

    #[test]
    fn test_properties_get_id_as_str() {
        let mut props = Properties::new();
        props.set_id(1);
        assert_eq!(props.get_id_as_str(), Some("1".to_string()));
    }
}