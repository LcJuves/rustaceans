#![allow(dead_code)]

/**
 * Created at 2021/7/30 10:44
 *
 * @author Liangcheng Juves
 */
use std::collections::HashMap;

pub struct Header {
    inner: HashMap<String, String>,
}

impl Header {
    pub fn new(map: HashMap<String, String>) -> Self {
        Header { inner: map }
    }

    pub fn add(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        self.inner.remove(&key);
    }

    pub fn get(&mut self, key: String) -> String {
        self.inner.get(&key).unwrap().to_string()
    }

    // pub fn parse(bytes: &[u8]) -> Self {
    //     let mut map = HashMap::<String, String>::new();
    //     Header { inner: map }
    // }
}
