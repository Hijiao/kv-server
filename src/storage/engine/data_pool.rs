use std::collections::BTreeMap;
use std::collections::btree_map::Range;
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


//    let v = b"v".to_vec();

//    da.insert(&k,v);

//    println!("k:{:?}",k)

//    da.insert("key_b", "value_b");
//    da.insert("key_c", "value_c");
}

//
//struct ScanIter {
//    cur_item: Option<(String, String)>,
//    da: DataPool,
//}
//
//impl ScanIter {
//    fn new(startKey: Key) -> ScanIter {
//        let mut da = DataPool::new();
//        da.insert(b"ka".to_vec(), b"va".to_vec());
//        da.insert(b"kb".to_vec(), b"vb".to_vec());
//        da.insert(b"kc".to_vec(), b"vc".to_vec());
//
//
//        ScanIter {
//            cur_item: Some((unsafe { &String::from_utf8_unchecked(startKey) }, &String::from(""))),
//            da,
//        }
//    }
//}
//
//impl Iterator for ScanIter {
//    type Item = (&String, &String);
//
//    fn next(&mut self) -> Option<(String, String)> {
//        match self.cur_item {
//            None => None,
//            Some((k, v)) => {
//                let mut it = self.da.find(k.into_bytes());
//                match it.next() {
//                    None => {
//                        self.cur_item = None;
//                        None
//                    }
//                    Some((k, v)) => {
//                        self.cur_item = Some((*k, *v));
//                        Some((k, v))
//                    }
//                }
//            }
//        }
//    }
//}

#[test]
fn data_pool_test() {
//    let mut da = DataPool::new();
//    da.insert(b"".to_vec(), b"v".to_vec());
//    da.show_all();
//
//    let mut iter = ScanIter::new(b"a".to_vec());
//
//    let n = iter.next();
//
//    println!("n: {:?}", n);


//    da.insert("key_a", "value_a");
//
//    da.insert("key_a", "value_aa");
//
//    da.insert("key_b", "value_b");
//    da.insert("key_c", "value_c");
//
//
//    da.show_all();
//    assert_eq!(da.btree.len(), 3);
//
//    da.delete("key_b");
//
//    assert_eq!(da.btree.len(), 2);
//    da.show_all();

//    let mut it = da.find("");
//
//    let n = it.next();
//
//    assert_eq!(Some((&"key_a", &"value_aa")), n);
//
//    let n = it.next();
//
//    assert_eq!(Some((&"key_c", &"value_c")), n);


//    let k = b"key_a".as_ref();
//
//    let mut kk = str::from_utf8(k).unwrap();
//    da.insert(kk, "dfvs");
//
//
//    let t = b"key_aa".to_vec();
//
//    let mut tt: String = unsafe { String::from_utf8_unchecked(t) };
//
//    da.insert(&*tt, "dfvs");
//
//    da.show_all();

//    use std::collections::BTreeMap;
//    use std::ops::Bound::Included;
//
//    let mut map = BTreeMap::new();
////    map.insert(3, "a");
//////    map.insert(5, "b");
//////    map.insert(8, "c");
//    for (&key, &value) in map.range((Included(&4), Included(&8))) {
//        println!("{}: {}", key, value);
//    }
//    assert_eq!(Some((&5, &"b")), map.range(4..).next());
}

