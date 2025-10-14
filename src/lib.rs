//#![allow(unused)]
use std::{ 
    thread,
    sync::{mpsc, Mutex, Arc },
};

//-----job
//Job type alias that will hold the closures we want to send down the channel.
type Job = Box<dyn FnOnce() + Send + 'static>;


//------------worker
//Define a Worker struct that holds an id and a JoinHandle<()>.
struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}



//-------job pool
// with mpsc::Sender, a channel to function as the queue of jobs, and execute will send a job from the ThreadPool to the Worker instances,
pub struct ThreadPool{
    //threads:: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}


//We chose usize as the type of the size paramete
//withn assert we validate the size is correct
//definition of ThreadPool to hold a vector of thread::JoinHandle<()> instances, initialized the vector with a capacity of size
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        //taking a job off the channel queue involves mutating the receiver
        //Arc type will let multiple Worker instances own the receiver
        //Mutex will ensure that only one Worker gets a job from the receiver at a time
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            //For each new Worker, we clone the Arc to bump the reference count so the Worker instances can share ownership of the receiver
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { 
            workers,
            sender: Some(sender),
         }
    }

    //Execution
    //We still use the () after FnOnce because this FnOnce represents a closure that takes no parameters and returns the unit type ()
    //Send to transfer the closure from one thread to another
    //'static because we don’t know how long the thread will take to execute
    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
            let job = Box::new(f);
            //after we create a job instance, we send the job to the sending end of the channel
            self.sender.as_ref().unwrap().send(job).unwrap();
    }

}

//Dropping sender closes the channel, which indicates no more messages will be sent.
impl Drop for ThreadPool {
    fn drop (&mut self){
        drop (self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down the worker {}", worker.id);
            //threads will finish when the ThreadPool drop implementation calls join on them
            worker.thread.join().unwrap();
        }
    }
}
