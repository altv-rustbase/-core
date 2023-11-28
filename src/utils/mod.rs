pub mod terminal;
pub mod config;
use chrono::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

// CODE

pub struct Utils {}
impl Utils {
    pub fn digit_format(number: u32) -> String {
        format!("{:02}", number)
    }

    pub fn get_local_time(is_seconds:bool) -> String {
        let _time = Local::now();

        let mut _time_str = String::new();
        _time_str.push_str(Utils::digit_format(_time.hour()).to_string().as_str());
        _time_str.push_str(":");
        _time_str.push_str(Utils::digit_format(_time.minute()).to_string().as_str());

        if is_seconds {
            _time_str.push_str(":");
            _time_str.push_str(Utils::digit_format(_time.second()).to_string().as_str());
        }

        _time_str
    }
}

pub struct CustomPool<Key, Value> where Key: Eq + Hash, {
    name: String,
    list: HashMap<Key, Value>,
}

impl<Key, Value> CustomPool<Key, Value> where Key: Eq + Hash, {
    pub fn new(name:String) -> Self {
        Self {
            name,
            list: HashMap::new(),
        }
    }

    fn get(&self, key:&Key) -> Option<&Value> {
        self.list.get(key)
    }

    fn has(&self, key:&Key) -> bool {
        self.list.contains_key(key)
    }

    fn add(&mut self, key:Key, value:Value) {
        if !self.has(&key) {
            self.list.insert(key, value);
        }
    }

    fn remove(&mut self, key:&Key) {
        self.list.remove(key);
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn length(&self) -> usize {
        self.list.len()
    }

    pub fn get_list(&self) -> &HashMap<Key, Value> {
        &self.list
    }

    pub fn get_next_id(&self) -> usize {
        self.length() + 1
    }

    pub fn add_public(&mut self, key:Key, value:Value) {
        self.add(key, value);
    }

    pub fn get_public(&self, key:&Key) -> Option<&Value> {
        self.get(key)
    }

    pub fn has_public(&self, key:&Key) -> bool {
        self.has(key)
    }

    pub fn remove_public(&mut self, key:&Key) {
        self.remove(key);
    }

    pub fn update_key_value(&mut self, key:Key, value:Value) {
        if self.has(&key) {
            self.remove(&key);
            self.add(key, value);
        }
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}
