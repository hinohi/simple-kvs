use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::RwLock;

use cmd::Cmd;

pub struct DB {
    n: usize,
    dbs: Vec<RwLock<HashMap<String, i64>>>,
}

impl DB {
    pub fn new(n: usize) -> Self {
        let mut dbs = Vec::new();
        for _ in 0..n {
            dbs.push(RwLock::new(HashMap::new()))
        }
        DB { n, dbs }
    }

    pub fn execute(&self, cmd: Cmd) -> Option<i64> {
        match cmd {
            Cmd::GET(key) => self.do_get(key),
            Cmd::SET(key, value) => self.do_set(key, value),
            Cmd::ADD(key, value) => self.do_add(key, value),
            Cmd::DELETE(key) => self.do_delete(key),
            Cmd::COUNT => self.do_count(),
        }
    }

    fn key_hash(&self, key: &str) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        s.finish() as usize % self.n
    }

    fn do_get(&self, key: String) -> Option<i64> {
        let db = self.dbs[self.key_hash(&key)].read().unwrap();
        db.get(&key).and_then(|v| Some(*v))
    }

    fn do_set(&self, key: String, value: i64) -> Option<i64> {
        let mut db = self.dbs[self.key_hash(&key)].write().unwrap();
        db.insert(key, value)
    }

    fn do_add(&self, key: String, value: i64) -> Option<i64> {
        let mut db = self.dbs[self.key_hash(&key)].write().unwrap();
        let v = db.entry(key).or_insert(0);
        *v += value;
        Some(*v)
    }

    fn do_delete(&self, key: String) -> Option<i64> {
        let mut db = self.dbs[self.key_hash(&key)].write().unwrap();
        db.remove(&key)
    }
    fn do_count(&self) -> Option<i64> {
        let mut count = 0;
        for (i, db) in self.dbs.iter().enumerate() {
            let db = db.read().unwrap();
            count += db.len();
            println!("DB size id={} count={}", i, db.len());
        }
        Some(count as i64)
    }
}
