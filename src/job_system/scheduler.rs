use core::num;
use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;
use std::thread::{self, JoinHandle};

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    task_queue: Arc<Mutex<VecDeque<Task>>>,
    condvar: Arc<Condvar>,
    shutdown: Arc<Mutex<bool>>,
}

impl ThreadPool {
    fn worker_loop(
        id: usize,
        task_queue: Arc<Mutex<VecDeque<Task>>>,
        condvar: Arc<Condvar>,
        shutdown: Arc<Mutex<bool>>,
    ) {
        loop {
            let task = {
                let mut queue = task_queue.lock().unwrap();

                // wait until filled
                while queue.is_empty() {
                    if *shutdown.lock().unwrap() {
                        return;
                    }

                    // 1. unlock queue
                    // 2. wait until notify
                    // 3. if wake then lock queue.
                    queue = condvar.wait(queue).unwrap();
                }

                queue.pop_front()
            };

            if let Some(task) = task {
                task();
            }
        }
    }

    pub fn new(num_workers: usize) -> Self {
        let task_queue = Arc::new(Mutex::new(VecDeque::new()));
        let condvar = Arc::new(Condvar::new());
        let shutdown = Arc::new(Mutex::new(false));

        let mut workers = Vec::with_capacity(num_workers);

        for id in 0..num_workers {
            let queue = Arc::clone(&task_queue);
            let cond = Arc::clone(&condvar);
            let shut = Arc::clone(&shutdown);

            let handle = thread::spawn(move || {
                Self::worker_loop(id, queue, cond, shut);
            });

            workers.push(handle);
        }

        Self { workers, task_queue, condvar, shutdown }
    }

    pub fn submit<F>(&self, task: F)
        where F: FnOnce() + Send + 'static
    {
        {
            let mut queue = self.task_queue.lock().unwrap();
            queue.push_back(Box::new(task));
        }

        self.condvar.notify_one();
    }

    pub fn submit_batch<F>(&self, tasks: Vec<F>)
        where  F: FnOnce() + Send + 'static
    {
        {
            let mut queue = self.task_queue.lock().unwrap();

            for task in tasks {
                queue.push_back(Box::new(task));
            }
        }

        self.condvar.notify_all();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        *self.shutdown.lock().unwrap() = true;

        self.condvar.notify_all();

        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
    }
}

/// Job System scheduler
/// Manages worker threads and distributes tasks
pub struct Scheduler {
    // TODO: Add fields
    // - worker handles
    // - task queues
    // - shutdown flag
}
