use std::collections::BTreeMap;
use std::collections::btree_map::Range;
use std::str;

pub struct DataPool {
    pub btree: BTreeMap<String, String>,
}


type Value = Vec<u8>;

type Key = Vec<u8>;

impl DataPool {
    pub fn new() -> Self {
        DataPool {
            btree: BTreeMap::new()
        }
    }
    pub fn insert(&mut self, k: Vec<u8>, v: Value) -> Option<String> {
        unsafe {
            self.btree.insert(String::from_utf8_unchecked(k), String::from_utf8_unchecked(v))
        }
    }

    pub fn get(&self, k: Vec<u8>) -> Option<&String> {
        unsafe { self.btree.get(&String::from_utf8_unchecked(k)) }
    }

    pub fn find(&self, k: Vec<u8>) -> Range<String, String> {
        unsafe { self.btree.range(String::from_utf8_unchecked(k)..) }
    }

    pub fn delete(&mut self, k: Vec<u8>) {
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


//    da.show_all();

    assert_eq!(da.btree.len(), 3);

    da.delete(b"key_b".to_vec());
    assert_eq!(da.btree.len(), 2);


//    let v = b"v".to_vec();

//    da.insert(&k,v);

//    println!("k:{:?}",k)

//    da.insert("key_b", "value_b");
//    da.insert("key_c", "value_c");
}


#[test]
fn data_pool_test() {
//    let mut da = DataPool::new();


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
}

