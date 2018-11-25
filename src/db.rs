use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::RwLock;

use query::Query;

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

    pub fn execute(&self, query: Query) -> Option<i64> {
        match query {
            Query::GET(key) => self.do_get(key),
            Query::SET(key, value) => self.do_set(key, value),
            Query::ADD(key, value) => self.do_add(key, value),
            Query::DELETE(key) => self.do_delete(key),
            Query::COUNT => self.do_count(),
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
        *v = v.wrapping_add(value);
        Some(*v)
    }

    fn do_delete(&self, key: String) -> Option<i64> {
        let mut db = self.dbs[self.key_hash(&key)].write().unwrap();
        db.remove(&key)
    }
    fn do_count(&self) -> Option<i64> {
        let mut count = 0;
        for db in &self.dbs {
            let db = db.read().unwrap();
            count += db.len();
        }
        Some(count as i64)
    }
}

#[cfg(test)]
mod tests {
    use query::Query;
    use db::DB;

    #[test]
    fn simple_cmd_execute() {
        let db = DB::new(1);
        assert_eq!(db.execute(Query::COUNT), Some(0));
        assert_eq!(db.execute(Query::GET("a".to_string())), None);
        assert_eq!(db.execute(Query::DELETE("a".to_string())), None);
        assert_eq!(db.execute(Query::SET("a".to_string(), 3)), None);
        assert_eq!(db.execute(Query::COUNT), Some(1));
        assert_eq!(db.execute(Query::GET("a".to_string())), Some(3));
        assert_eq!(db.execute(Query::SET("b".to_string(), 10)), None);
        assert_eq!(db.execute(Query::COUNT), Some(2));
        assert_eq!(db.execute(Query::GET("b".to_string())), Some(10));
        assert_eq!(db.execute(Query::SET("b".to_string(), -42)), Some(10));
        assert_eq!(db.execute(Query::COUNT), Some(2));
        assert_eq!(db.execute(Query::GET("b".to_string())), Some(-42));
        assert_eq!(db.execute(Query::DELETE("a".to_string())), Some(3));
        assert_eq!(db.execute(Query::COUNT), Some(1));
        assert_eq!(db.execute(Query::GET("a".to_string())), None);
        assert_eq!(db.execute(Query::GET("b".to_string())), Some(-42));
        assert_eq!(db.execute(Query::ADD("b".to_string(), 100)), Some(58));
        assert_eq!(db.execute(Query::COUNT), Some(1));
        assert_eq!(db.execute(Query::GET("b".to_string())), Some(58));
        assert_eq!(db.execute(Query::ADD("b".to_string(), 0)), Some(58));
        assert_eq!(db.execute(Query::ADD("c".to_string(), 0)), Some(0));
        assert_eq!(db.execute(Query::COUNT), Some(2));
        assert_eq!(db.execute(Query::GET("b".to_string())), Some(58));
        assert_eq!(db.execute(Query::GET("c".to_string())), Some(0));
        assert_eq!(db.execute(Query::ADD("c".to_string(), -1)), Some(-1));
        assert_eq!(db.execute(Query::COUNT), Some(2));
        assert_eq!(db.execute(Query::GET("c".to_string())), Some(-1));
    }
    #[test]
    fn overflow_add() {
        use std::i64;
        let db = DB::new(1);
        db.execute(Query::SET("key".to_string(), i64::MAX));
        assert_eq!(db.execute(Query::ADD("key".to_string(), 1)), Some(i64::MIN));
    }
}
