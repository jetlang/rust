use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

pub enum Events<T: 'static> {
    Task(Box<Fn()->bool+Send>),
    Data(T)
}

pub struct Fiber<T: 'static> {
    sender: Sender<Events<T>>,
    t: std::thread::JoinHandle<()>
}

impl <T: Send> Fiber<T> {
    pub fn new<F>(fun: F) -> Fiber<T>
        where  F: Send + 'static + Fn(T),{
        let (tx, rx): (Sender<Events<T>>, Receiver<Events<T>>) = mpsc::channel();
        return Fiber::new_from_channel(fun, tx, rx);
    }

    pub fn new_from_channel<F>(fun:F, sen:Sender<Events<T>>, rcv: Receiver<Events<T>>) -> Fiber<T>
        where  F: Send + 'static + Fn(T),{
            let t = thread::spawn (move || {
                let mut running = true;
                while running {
                    let event = rcv.recv().unwrap();
                    match event {
                        Events::Task(t) => running = t(),
                        Events::Data(d) => fun(d)
                    }
                }
            });
            return Fiber{sender:sen, t:t};
        }


    pub fn send(&self, msg:Events<T>) {
        self.sender.send(msg).unwrap();
    }

    pub fn send_data(&self, f: T){
        self.send(Events::Data(f));
    }

    pub fn send_stop(&self) {
        let end = move || {
            return false;
        };
        self.send(Events::Task(Box::new(end)));
    }

    pub fn join(self) {
        self.t.join().unwrap();
    }
}