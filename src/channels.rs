use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

trait Event : Sized {
    fn run();
}

fn main() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    let (tx, rx): (Sender<Box<Fn()+Send>>, Receiver<Box<Fn()+Send>>) = mpsc::channel();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        thread::spawn(move || {
            let printer = move || println!("{:?}", id);
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(Box::new(printer)).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            //println!("thread {} finished", id);
        });
    }

    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there no messages available
        rx.recv().unwrap()();
    }

    // Show the order in which the messages were sent
    //println!("{:?}", ids);
}