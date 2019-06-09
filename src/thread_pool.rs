use std::thread::JoinHandle;
use std::thread;

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>
}

impl ThreadPool {
    pub fn spawn<F>(&mut self, f: F) where
        F: FnOnce(), F: Send + 'static
    {
        self.threads.push(thread::spawn(f));
    }

    pub fn wait(self) {
        for thread in self.threads {
            let _ = thread.join();
        }
    }
}

impl Default for ThreadPool {
    fn default() -> Self { ThreadPool{threads: Vec::new()} }
}
