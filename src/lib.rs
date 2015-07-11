use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

#[allow(dead_code)]
enum Events<T: 'static> {
    Task(Box<Fn()->bool+Send>),
    Data(T)
}

#[allow(dead_code)]
struct Fiber<T: 'static> {
    sender: Sender<Events<T>>,
    t: std::thread::JoinHandle<()>
}

#[allow(dead_code)]
impl <T: Send> Fiber<T> {
    fn new<F>(fun: F) -> Fiber<T>
        where  F: Send + 'static + Fn(T),{
        let (tx, rx): (Sender<Events<T>>, Receiver<Events<T>>) = mpsc::channel();
        let t = thread::spawn (move || {
            let mut running = true;
            while running {
                let event = rx.recv().unwrap();
                match event {
                    Events::Task(t) => running = t(),
                    Events::Data(d) => fun(d)
                }
            }
        });
        return Fiber{sender:tx, t:t};
    }

    fn send(&self, msg:Events<T>) {
        self.sender.send(msg).unwrap();
    }

    fn send_stop(&self) {
        let end = move || {
            return false;
        };
        self.send(Events::Task(Box::new(end)));
    }

    fn join(self) {
        self.t.join().unwrap();
    }
}
#[test]
fn basic() {
    let mut vec = Vec::new();
    for id in 0..3 {
        let rcv_loop = |data| {
            println!("{:?}", data);
        };
        let f = Fiber::new(rcv_loop);
        let printer = move || {
            println!("{:?}", id);
            return true;
        };
        f.send(Events::Task(Box::new(printer)));
        f.send(Events::Data(id + 1000));
        vec.push(f);
    }

    for f in &vec {
        f.send_stop();
    }

    for g in vec {
        g.join();
    }
}
