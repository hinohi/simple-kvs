use std::collections::HashMap;
use std::sync::RwLock;

use cmd::Cmd;

pub struct DB {
    db: RwLock<HashMap<String, i64>>,
}

impl DB {
    pub fn new() -> Self {
        DB {
            db: RwLock::new(HashMap::new()),
        }
    }

    pub fn execute(&self, cmd: Cmd) -> Option<i64> {
        match cmd {
            Cmd::GET(key) => self.do_get(key),
            Cmd::SET(key, value) => self.do_set(key, value),
            Cmd::ADD(key, value) => self.do_add(key, value),
            Cmd::DELETE(key) => self.do_delete(key),
        }
    }

    fn do_get(&self, key: String) -> Option<i64> {
        let db = self.db.read().unwrap();
        db.get(&key).and_then(|v| Some(*v))
    }

    fn do_set(&self, key: String, value: i64) -> Option<i64> {
        let mut db = self.db.write().unwrap();
        db.insert(key, value)
    }

    fn do_add(&self, key: String, value: i64) -> Option<i64> {
        let mut db = self.db.write().unwrap();
        let v = db.entry(key).or_insert(0);
        *v += value;
        Some(*v)
    }

    fn do_delete(&self, key: String) -> Option<i64> {
        let mut db = self.db.write().unwrap();
        db.remove(&key)
    }
}
