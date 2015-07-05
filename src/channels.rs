use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

struct Fiber {
    sender: Sender<Box<Fn()->bool+Send>>,
}

impl Fiber {
    fn new() -> Fiber {
        let (tx, rx): (Sender<Box<Fn()->bool+Send>>, Receiver<Box<Fn()->bool+Send>>) = mpsc::channel();
        let f= Fiber{sender:tx};
        thread::spawn (move || {
            while rx.recv().unwrap()() {
            }
        });
        return f;
    }

    fn stop(&self) {
        let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
        let end = move || {
            tx.send(true).unwrap();
            return false;
        };
        self.sender.send(Box::new(end)).unwrap();
        rx.recv().unwrap();
    }
}

fn main() {
    let mut vec = Vec::new();
    for id in 0..NTHREADS {
        let f = Fiber::new();
        let printer = move || {
            println!("{:?}", id);
            return true;
        };
        f.sender.send(Box::new(printer)).unwrap();
        vec.push(f);
    }

    for f in vec {
        f.stop();
    }
}