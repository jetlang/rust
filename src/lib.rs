use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

pub enum Events<T: 'static> {
    Stop,
    Data(T)
}

pub struct Fiber<T: 'static> {
    sender: Sender<Events<T>>,
    t: std::thread::JoinHandle<()>
}

impl <T: Send> Fiber<T> {
    pub fn new<F, G>(fun: F) -> Fiber<T>
        where
        G: Send + 'static + FnMut(Events<T>)->bool,
        F: Send + 'static + FnOnce()->G,{
        let (tx, rx): (Sender<Events<T>>, Receiver<Events<T>>) = mpsc::channel();
            let t = thread::spawn (move || {
                let mut running = true;
                let mut runner = fun();
                while running {
                    let event = rx.recv().unwrap();
                    running = runner(event);
                }
            });
            Fiber{sender:tx, t:t}
        }

    pub fn clone_sender(&self)->Sender<Events<T>>{
        self.sender.clone()
     }


    pub fn send(&self, msg:Events<T>) {
        self.sender.send(msg).unwrap();
    }

    pub fn send_data(&self, f: T){
        self.send(Events::Data(f));
    }

    pub fn send_stop(&self) {
        self.send(Events::Stop);
    }

    pub fn join(self) {
        self.t.join().unwrap();
    }
}