use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, Thread},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// size is number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// panics if size is equal to 0
    pub fn new (size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{ workers, sender }
    }

    pub fn execute<F> (&self, f: F) 
    where 
        F: FnOnce() + Send + 'static 
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker{ id, thread }
    }
}