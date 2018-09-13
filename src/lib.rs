use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>
}

trait FnBox {
  fn call_box(self: Box<Self>);
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    
    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F) 
    where F: FnOnce() + Send + 'static
    {
      let job = Box::new(f);
      self.sender.send(job).unwrap();
    }
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    Worker {
      id,
      thread: thread::spawn(move || {
        loop {
          let job = receiver.lock().unwrap().recv().unwrap();

          println!("Worker {} got a job!", id);

          job.call_box();
        }
      })
    }
  }
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    (*self)()
  }
}