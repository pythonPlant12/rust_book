use std::sync::{ mpsc, Arc, Mutex };
use std::thread::JoinHandle;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job; executing.");
                job();
            }
        });

        Worker { id, thread }
    }
}
type Job = Box<dyn FnOnce() + Send + 'static>;

// 1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>` DONE
// 2. Change `ThreadPool` to hold a vector of `Worker` instances. DONE
// 3. Define a `Worker::new` function that take an `id` number and returns a `Worker` instance that holds the `id`
// and a thread spawned with an empty closure. DONE
// 4. In `ThreadPool::new`, use the `for` loop counter to generate an `id`, create a new `Worker` with that
// id, and store the worker in the vector.

// Next what we need to do:
// 1. The `ThreadPool` will create a channel and hold on to the sender.
// 2. Each `Worker` will hold on to the receiver
// 3. We'll create a new `Job` struct that will hold the closures we want to send down the channel.
// 4. The `execute` method will send the job it wants to execute through the sender.
// 5. In its thread, the `Worker` will loop over its receiver and execute the closures of any jobs it receives.

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // let worker = Worker::new(id, receiver); // This will not work as `receiver` or consumer, is mpsc (single consumer)
            // And we move it in first iteration
            // So now the workers can share the ownership of the receiver using `Arc<Mutex<>>`
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
        }
        ThreadPool { workers, sender }
    }

    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    //     where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static {}

    // Let's finally implement the `execute` method on `ThreadPool`
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}
