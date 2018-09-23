extern crate kvlib;

use kvlib::storage::engine::data_pool::{DataPool};

//fn dpmain() {
//    let mut dp = DataPool::new();
//
////    let k = b"avc".to_vec();
////    let v = b"kkk".to_vec();
//
//
//    dp.insert(b"key_a".to_vec(), b"value_a".to_vec());
//
//    dp.insert(b"key_a".to_vec(), b"value_aa".to_vec());
//
//    dp.insert(b"key_b".to_vec(), b"value_b".to_vec());
//
//    dp.insert(b"key_c".to_vec(), b"value_c".to_vec());
//    dp.insert(b"key_d".to_vec(), b"value_d".to_vec());
////
////    for (key, value) in dp.btree.range("key_b".to_string()..) {
////        println!("{}: {}", key, value);
////    }
//
//    let mut ran = dp.btree.range("key_b".to_string()..);
//    let r = ran.next();
//    assert_eq!(Some(("key_b".as, "value_b")), r);
////
////
////    let (k, v) = ran.next().unwrap();
////    println!("{}: {}", k, v);
//
//
//    dp.show_all();
//}

fn main() {
    let mut da = DataPool::new();
    da.insert("key_a", "value_a");

    da.insert("key_a", "value_aa");

    da.insert("key_b", "value_b");

    da.show_all();
}

