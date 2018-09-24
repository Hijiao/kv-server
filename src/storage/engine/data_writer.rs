extern crate crossbeam_channel;

use crossbeam_channel::{Sender, Receiver};
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::{io, usize};
use std::thread;
use std::fmt::Display;


const DEFAULT_WRITE_BATCH_SIZE: usize = 1;

//const DEFAULT_WRITE_PENDING_SIZE: usize = usize::MAX;
const DEFAULT_WRITE_PENDING_SIZE: usize = 1024;

pub struct WriteQueue<T> {
    sender: Sender<Option<T>>,
}

impl<T> WriteQueue<T> {
    fn new(sender: Sender<Option<T>>) -> WriteQueue<T> {
        WriteQueue {
            sender,
        }
    }

    pub fn append(&self, task: Option<T>) {
        self.sender.send(task);
    }
}

impl<T> Clone for WriteQueue<T> {
    fn clone(&self) -> WriteQueue<T> {
        WriteQueue {
            sender: self.sender.clone()
        }
    }
}


pub struct DataWriter<T> {
    queue: WriteQueue<T>,
    receiver: Mutex<Option<Receiver<Option<T>>>>,
    handle: Option<JoinHandle<()>>,
    batch_size: usize,
    pending_size: usize,
}

impl<T: Send + Display + 'static> DataWriter<T> {
    pub fn new() -> DataWriter<T> {
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

    pub fn get_queue(&self) -> WriteQueue<T> {
        self.queue.clone()
    }

    pub fn stop(&self) {
        self.queue.append(None);
    }
}

fn poll_to_write<T: Display>(rx: Receiver<Option<T>>, batch_size: usize) {
    let mut batch = Vec::with_capacity(batch_size);
    let mut keep_going = true;
    while keep_going {
        println!("do one loop");
        keep_going = fill_task_batch(&rx, &mut batch, batch_size);
        if !batch.is_empty() {
            write_batch(&mut batch);
            batch.clear();
        }
    }
    println!("write stop polling")
}


fn write_batch<T: Display>(batch: &mut Vec<T>) {
    for t in batch.drain(..) {
        println!("write to disk");
        println!("write to disk :t = {}", t);
    }
}


fn fill_task_batch<T>(rx: &Receiver<Option<T>>, buffer: &mut Vec<T>, batch_size: usize) -> bool {
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
    let mut writer: DataWriter<i32> = DataWriter::new();
//    let queue = WriteQueue::new(writer.clone());
    writer.start();
    writer.queue.append(Some(1));
    writer.queue.append(Some(2));
    thread::sleep(Duration::from_millis(100));


//    let _ = io::stdin().read(&mut [0]).unwrap();
    writer.stop();
}