extern crate core;

use std::{fmt, thread};
use std::fmt::Formatter;
use std::sync::{Arc, mpsc, Mutex};
use std::thread::JoinHandle;

pub struct ControllerResp {
    core: usize,
    msg: String,
}

impl fmt::Debug for ControllerResp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.core {
            404 => write!(f, "错误类型:{},错误原因:{}", self.core, self.msg),
            _ => write!(f, "无法识别的错误")
        }
    }
}

impl fmt::Display for ControllerResp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.core {
            404 => write!(f, "错误类型:{},错误原因:{}", self.core, self.msg),
            _ => write!(f, "无法识别的错误")
        }
    }
}


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rc: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = rc.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing",id);
            job();
        });
        Worker {
            id,
            thread,
        }
    }
}

pub struct ThreadPool {
    num: usize,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(num: usize) -> Result<ThreadPool, ControllerResp> {
        if num <= 0 {
            Err(ControllerResp { core: 505, msg: String::from("创建线程池异常") })
        } else {
            let mut workers = Vec::with_capacity(num);

            let (send, rc) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(rc));

            for i in 0..num {
                let work = Worker::new(i, receiver.clone());
                workers.push(work);
            }

            Ok(ThreadPool {
                num,
                workers,
                sender: send,
            })
        }
    }

    pub fn exec<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

