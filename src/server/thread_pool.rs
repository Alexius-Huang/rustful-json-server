use std::{
    sync::{
        mpsc::{self, Receiver},
        Mutex,
        Arc
    },
    thread::{self, JoinHandle}
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    _id: usize,
    thread: Option<JoinHandle<()>>
}

pub struct PoolCreationError(String);

impl ThreadPool {
    /// Creates a ThreadPool
    /// 
    /// The capacity is the highest number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is 0
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);

        let mut workers = Vec::with_capacity(capacity);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..capacity {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // No more message/job would be sent when shutting down server
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => job(),        
                Err(_) => break
            }
        });

        Self { _id: id, thread: Some(thread) }
    }
}
