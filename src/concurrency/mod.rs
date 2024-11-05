#![allow(unused_variables, dead_code)]

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use rand::Rng;

pub fn demo_spawning_threads() {
    println!("\nDemo spawning threads");

    thread::spawn(|| {
        for i in 1..=10 {
            println!("{:?} displaying {}", thread::current().id(), i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 100..=105 {
        println!("{:?} displaying {}", thread::current().id(), i);
        thread::sleep(Duration::from_secs(4));
    }

    println!("That's all, folks!");
}

pub fn demo_joining_thread_single() {
    println!("\nDemo joining thread single");

    let handle = thread::spawn(|| {
        println!("{:?} starting", thread::current().id());

        thread::sleep(Duration::from_secs(10));

        println!("{:?} ending", thread::current().id());

        // panic!("Deliberate panicking, dude!")
    });


    for i in 100..=105 {
        println!("{:?} displaying {}", thread::current().id(), i);
        thread::sleep(Duration::from_millis(500));
    }

    println!("{:?} waiting for other thread to end", thread::current().id());

    // handle.join().unwrap();

    match handle.join()     {
        Ok(_) => println!("join() result is Ok"),
        Err(_) => println!("join() result is Err"),
    }

    println!("That's all, folks!");
}

pub fn demo_joining_thread_multiple() {
    println!("\nDemo joining thread multiple");

    let mut handles: Vec::<thread::JoinHandle::<_>> = vec![];

    for _ in 1..=5 {
        let handle = thread::spawn(|| {
            let mut rng = rand::thread_rng();
            let num = rng.gen_range(5..10);

            println!("{:?} sleep for {} sec - starting", thread::current().id(), num);
            thread::sleep(Duration::from_secs(num));
            println!("{:?} sleep for {} sec - ended", thread::current().id(), num);
        });
        handles.push(handle);
    }

    for i in 100..=105 {
        println!("{:?} displaying {}", thread::current().id(), i);
        thread::sleep(Duration::from_millis(500));
    }

    println!("{:?} waiting for other threads to end", thread::current().id());

    for h in handles {
        h.join().unwrap();
    }

    println!("That's all, folks!");
}

pub fn demo_capturing_state_implicit_move() {
    println!("\nDemo capturing state implicit move");

    let handle = do_some_work_implicit_move();
    handle.join().unwrap();

    println!("That's all, folks!");
}

pub fn demo_capturing_state_explicit_move() {
    println!("\nDemo capturing state explicit move");

    let handle = do_some_work_explicit_move();
    handle.join().unwrap();

    println!("That's all, folks!");
}

pub fn demo_channels_single_message() {
    println!("\nDemo channels single message");

    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        tx.send(String::from("Hei hei")).unwrap();
    });

    let received = rx.recv().unwrap();

    println!("Received {}", received);

    handle.join().unwrap();
}

pub fn demo_channels_multiple_messages() {
    println!("\nDemo channels multiple messages");

    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        for i in 1..=10 {
            let s = std::format!("Message {}", i);
            println!("Sending: {}", s);
            tx.send(s).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }

    handle.join().unwrap();
}

fn do_some_work_implicit_move() -> thread::JoinHandle<()> {

    let v = vec![100, 101, 102, 103, 104, 105];

    let handle = thread::spawn(|| {
        for item in v {
            println!("{:?} displaying {}", thread::current().id(), item);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // v was moved implicitly to the closure
    // println!("{:?} displaying {:?}", thread::current().id(), v);

    handle
}

fn do_some_work_explicit_move() -> thread::JoinHandle<()> {

    let v = vec![100, 101, 102, 103, 104, 105];

    let handle = thread::spawn(move || {
        for item in &v {
            println!("{:?} displaying {}", thread::current().id(), item);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // v was moved implicitly to the closure
    // println!("{:?} displaying {:?}", thread::current().id(), v);

    handle
}









