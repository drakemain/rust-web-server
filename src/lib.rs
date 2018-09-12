use std::thread;

pub struct ThreadPool {
  threads: Vec<thread::JoinHandle<()>>
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let threads = Vec::with_capacity(size);

    for _ in 0..size {
      
    }
    
    ThreadPool { threads }
  }

  pub fn execute<F>(&self, f: F) 
    where F: FnOnce() + Send + 'static
    {
      
    }
}