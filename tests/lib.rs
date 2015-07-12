extern crate jetlang;

use jetlang::{Fiber, Events};
use std::thread;

#[test]
fn basic() {
    let mut vec = Vec::new();
    for id in 0..3 {
        let runner = move || {
            let mut vSize :Vec<i32> = Vec::new();
            return move |data: Events<i32>| {
                match data {
                    Events::Stop=> return false,
                    Events::Data(d)=> {
                        vSize.push(d);
                        println!("{:?}", d);
                        println!("{:?}", vSize.len());
                        return true;
                    }
                }
                };
        };
        let f:Fiber<i32> = Fiber::new(runner);
        f.send_data((id + 1000));
        vec.push(f);
    }

    for f in &vec {
        f.send_stop();
    }

    for g in vec {
        g.join();
    }
}

#[test]
fn spawn(){
    let f = |vec: &mut Vec<i32>, id: i32| {
        vec.push(id);
    };

    let mut v: Vec<i32> = Vec::new();
    let counter = move|| {
        for id in 0..3 {
            f(&mut v, id);
        }
    };
    let t = thread::spawn(counter);
    t.join().unwrap();
}

#[test]
fn function_passing(){
    let mut v: Vec<i32> = Vec::new();
    let f = move || {
        return move |i:i32| v.push(i);
    };

    let runner = move|| {
        let mut c = f();
        for id in 0..3 {
              c(id);
        }
    };
    let t = thread::spawn(runner);
    t.join().unwrap();
}

