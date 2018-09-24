extern crate crossbeam_channel;

use crossbeam_channel::{Sender, Receiver};
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::{io, usize};
use std::thread;
use std::fmt::Display;
use super::Task;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;

const DEFAULT_WRITE_BATCH_SIZE: usize = 4;

//const DEFAULT_WRITE_PENDING_SIZE: usize = usize::MAX;
const DEFAULT_WRITE_PENDING_SIZE: usize = 1024;
// '\n'
const FILE_SPLIT_MARK: [u8; 1] = [10];
// '0 /n'
const FILE_PUT_MARK: [u8; 2] = [48, 10];
// '1 /n'
const FILE_DELETE_MARK: [u8; 2] = [49, 10];

pub struct WriteQueue {
    sender: Sender<Option<Task>>,
}

impl WriteQueue {
    fn new(sender: Sender<Option<Task>>) -> WriteQueue {
        WriteQueue {
            sender,
        }
    }

    pub fn append(&self, task: Option<Task>) {
        self.sender.send(task);
    }
}

impl Clone for WriteQueue {
    fn clone(&self) -> WriteQueue {
        WriteQueue {
            sender: self.sender.clone()
        }
    }
}


pub struct DataWriter {
    queue: WriteQueue,
    receiver: Mutex<Option<Receiver<Option<Task>>>>,
    handle: Option<JoinHandle<()>>,
    batch_size: usize,
    pending_size: usize,
}

impl DataWriter {
    pub fn new() -> DataWriter {
        let batch_size = DEFAULT_WRITE_BATCH_SIZE;
        let pending_size = DEFAULT_WRITE_PENDING_SIZE;
        let (tx, rx) = crossbeam_channel::bounded(pending_size);
        DataWriter {
            queue: WriteQueue::new(tx),
            receiver: Mutex::new(Some(rx)),
            handle: None,
            batch_size,
            pending_size,
        }
    }

    pub fn start(&mut self) -> Result<(), io::Error> {
        let mut receiver = self.receiver.lock().unwrap();
        println!("starting writing thread");
        if receiver.is_none() {
            println!("writing thread  has been started.");
            return Ok(());
        }

        let batch_size = self.batch_size;
        let rx = receiver.take().unwrap();

        let h = thread::Builder::new()
            .name("write thread".to_string())
            .spawn(move || poll_to_write(rx, batch_size))?;
        self.handle = Some(h);
        Ok(())
    }

    pub fn get_queue(&self) -> WriteQueue {
        self.queue.clone()
    }

    pub fn stop(&self) {
        self.queue.append(None);
    }
}

fn poll_to_write(rx: Receiver<Option<Task>>, batch_size: usize) {
    let mut batch = Vec::with_capacity(batch_size);
    let mut keep_going = true;
    let mut file = OpenOptions::new().create(true).append(true).open("foo-1.txt").unwrap();
    while keep_going {
        keep_going = fill_task_batch(&rx, &mut batch, batch_size);
        if !batch.is_empty() {
            write_batch(&mut batch, &mut file);
            batch.clear();
        }
    }
}


fn write_batch(batch: &mut Vec<Task>, file: &mut File) {
    for t in batch.drain(..) {
        match t {
            Task::Put(k, v) => {
                file.write(&FILE_PUT_MARK);
                file.write(&k);
                file.write(&FILE_SPLIT_MARK);
                file.write(&v);
                file.write(&FILE_SPLIT_MARK);
            }
            Task::Delete(k) => {
                file.write(&FILE_DELETE_MARK);
                file.write(&k);
                file.write(&FILE_SPLIT_MARK);
            }
        }
    }
    file.sync_data();//? file.flush();
}


fn fill_task_batch(rx: &Receiver<Option<Task>>, buffer: &mut Vec<Task>, batch_size: usize) -> bool {
    let head_task = match rx.recv() {
        Some(msg) => {
            match msg {
                Some(task) => task,
                None => return false,
            }
        }
        None => return false,
    };
    buffer.push(head_task);

    while buffer.len() < batch_size {
        match rx.try_recv() {
            Some(msg) => {
                match msg {
                    Some(t) => buffer.push(t),
                    None => return false,
                }
            }
            None => return false,
        }
    }
    true
}

#[test]
fn data_write_test() {
    use std::io::Read;
    use std::time::Duration;
    let mut writer: DataWriter = DataWriter::new();

    writer.start();

    let p = Task::Put(b"kk".to_vec(), b"vv".to_vec());
    let d = Task::Delete(b"del-ket".to_vec());
    writer.queue.append(Some(p));
    writer.queue.append(Some(d));
    thread::sleep(Duration::from_millis(100));

//    let _ = io::stdin().read(&mut [0]).unwrap();
    writer.stop();
}

#[test]
fn write_batch_test() {
    let p = Task::Put(b"kk".to_vec(), b"vv".to_vec());
    let d = Task::Delete(b"del-ket".to_vec());

    {
        let mut ov: Vec<Task> = Vec::new();
        ov.push(p.clone());
        ov.push(d.clone());
        let mut file = OpenOptions::new().write(true).open("foo.txt").unwrap();
        write_batch(&mut ov, &mut file);
    }

    let mut nv: Vec<Task> = Vec::new();

    {
        let mut f = File::open("foo.txt").unwrap();
        let mut file = BufReader::new(&f);
        let mut it = file.lines();


        let mut line = it.next();

        while line.is_some() {
            let l = line.unwrap().unwrap();
            if l == "0" {
                let key = it.next().unwrap().unwrap();
                let val = it.next().unwrap().unwrap();
                nv.push(Task::Put(key.into_bytes(), val.into_bytes()))
            } else if l == "1" {
                let key = it.next().unwrap().unwrap();
                nv.push(Task::Delete(key.into_bytes()));
            }

            line = it.next()
        }
    }

    let ov = vec![p, d];

    assert_eq!(ov, nv)
}
