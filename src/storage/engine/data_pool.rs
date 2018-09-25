use std::collections::BTreeMap;
use super::{Key, Value};


//#[derive(Clone)]
pub struct DataPool {
    btree: BTreeMap<String, String>,
}


impl DataPool {
    pub fn new() -> Self {
        DataPool {
            btree: BTreeMap::new()
        }
    }
    pub fn insert(&mut self, k: Key, v: Value) -> Option<String> {
        unsafe {
            self.btree.insert(String::from_utf8_unchecked(k), String::from_utf8_unchecked(v))
        }
    }

    pub fn get(&self, k: Key) -> Option<String> {
        unsafe {
            let ret = self.btree.get(&String::from_utf8_unchecked(k));
            match ret {
                Some(s) => Some(s.to_owned()),
                None => None
            }
        }
    }

    pub fn find_next(&self, k: Key, next: bool) -> Option<(String, String)> {
        unsafe {
            let mut range = self.btree.range(String::from_utf8_unchecked(k)..);
            let wanted = if next {
                range.next();
                range.next()
            } else {
                range.next()
            };
            match wanted {
                None => None,
                Some((ref k, ref v)) => Some((k.clone().to_string(), v.clone().to_string()))
            }
        }
    }

    pub fn delete(&mut self, k: Key) {
        unsafe { self.btree.remove(&String::from_utf8_unchecked(k)) };
    }

    pub fn show_all(&mut self) {
        for (k, v) in &self.btree {
            println!("{:?}: {:?}", k, v);
        }
    }
}

//pub struct DataPool<'a> {
//    pub btree: BTreeMap<&'a str, &'a str>,
//}

//impl<'a,'b:'a> DataPool<'a> {
//    pub fn new() -> DataPool<'a> {
//        DataPool {
//            btree: BTreeMap::new()
//        }
//    }
//    pub fn insert(&'a mut self, k: &'b str, v: &'b str) {
//        self.btree.insert(k, v);
//    }
//
////    pub fn get(&self, k: &str) -> Option<&str> {
////        self.btree.get(k)
////    }
//
//    pub fn show_all(&self) {
//        for (k, v) in &self.btree {
//            println!("{:?}: {:?}", k, v);
//        }
//    }
//
//    pub fn find(&self, k: &'a str) -> Range<&str, &str> {
//        self.btree.range(k..)
//    }
//
//    pub fn delete(&mut self, k: &str) {
//        self.btree.remove(k);
//    }
//}

#[test]
fn data_pool_string_test() {
    let mut da = DataPool::new();
    da.insert(b"key_a".to_vec(), b"value_a".to_vec());

    da.insert(b"key_a".to_vec(), b"value_aa".to_vec());
    da.insert(b"key_b".to_vec(), b"value_b".to_vec());

    da.insert(b"key_c".to_vec(), b"value_c".to_vec());


    assert_eq!(da.btree.len(), 3);

    let search_key = b"key_b".to_vec();

    let r = da.get(search_key.clone()).unwrap();
    assert_eq!("value_b", r);

    da.delete(search_key.clone());
    assert_eq!(da.btree.len(), 2);

    let r = da.get(search_key.clone());
    assert_eq!(None, r);
    println!("search result:{:?}", r);

}
