use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));
        for index in 0..size {
            let worker = Worker::new(index, receiver.clone());
            workers.push(worker);
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send,
    {
        println!("sending job to threadpool...");
        self.sender.send(Message::NewJob(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("dropping all workers in pool...");
        for worker in &self.workers {
            println!("sending terminate message to Worker {}", worker.id);
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("waiting all workers to stop...");
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
                println!("Worker {} has stopped.", worker.id);
            }
        }

        println!("thread pool has terminated.");
    }
}

type Job = Box<dyn FnOnce() + 'static + Send>;

enum Message {
    Terminate,
    NewJob(Job),
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
                Message::NewJob(job) => {
                    println!("Worker {} is doing his job...", id);
                    job();
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
