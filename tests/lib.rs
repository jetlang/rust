extern crate jetlang;

use jetlang::{Fiber, Events};

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

