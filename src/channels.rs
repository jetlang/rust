use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

enum Events<T: 'static> {
    Task(Box<Fn()->bool+Send>),
    Data(T)
}

struct Fiber<T: 'static> {
    sender: Sender<Events<T>>
}

impl <T: Send> Fiber<T> {
    fn new(fun: Box<Fn(T) + Send>) -> Fiber<T> {
        let (tx, rx): (Sender<Events<T>>, Receiver<Events<T>>) = mpsc::channel();
        let f= Fiber{sender:tx};
        thread::spawn (move || {
            let mut running = true;
            while running {
                let event = rx.recv().unwrap();
                match event {
                    Events::Task(t) => running = t(),
                    Events::Data(d) => fun(d)
                }
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
        self.sender.send(Events::Task(Box::new(end))).unwrap();
        rx.recv().unwrap();
    }
}

fn main() {
    let mut vec = Vec::new();
    for id in 0..NTHREADS {
        let rcv_loop = |data| {
            println!("{:?}", data);
        };
        let f: (Fiber<i32>) = Fiber::new(Box::new(rcv_loop));
        let printer = move || {
            println!("{:?}", id);
            return true;
        };
        f.sender.send(Events::Task(Box::new(printer))).unwrap();
        f.sender.send(Events::Data(id + 1000)).unwrap();
        vec.push(f);
    }

    for f in vec {
        f.stop();
    }
}