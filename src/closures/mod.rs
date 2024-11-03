use std::thread::sleep;
use std::time::Duration;
use chrono::{DateTime, Utc};

pub fn demo_nested_functions() {
    println!("\nDemo nested functions");

    fn sqr(i: i32) -> i32 {
        i * i
    }

    println!("Square of 5 is {}", sqr(5));
    println!("Square of 7 is {}", sqr(7));
}

pub fn demo_closures() {
    println!("\nDemo closures");

    closure_no_params();
    closure_one_param();
    closure_many_params();
    closure_multiple_statements();
}

pub fn demo_closures_inferred_types() {
    println!("\nDemo closures in inferred types");
    closure_no_params_inferred_types();
    closure_one_param_inferred_types();
    closure_many_params_inferred_types();
    closure_multiple_statements_inferred_types();

}

pub fn demo_closures_capture_reference(){
    println!("\nDemo closures capture reference");

    capture_immutable_reference();
    capture_mutable_reference();
}

pub fn demo_closures_capture_value(){
    println!("\nDemo closures capture value");
    capture_value_automatically();
    capture_value_forcibly();

    std::thread::sleep(Duration::new(10, 0));
}

fn capture_value_automatically() {
    let message = String::from("hello");

    println!("Message initially: {}", message);

    let consume_message = || {
        println!("Message in closure: {}", message);
        std::mem::drop(message);
    };

    // No, message owns by closure
    //println!("message: {}", message);

    consume_message();
    // No, can be used only once
    // consume_message();
}


fn capture_value_forcibly() {
    let message = String::from("HELLO");
    println!("Start the method...");

    std::thread::spawn(move || {
        println!("Message at start of closure: {}", message);
        std::thread::sleep(Duration::new(5, 0));
        println!("Message at end of closure: {}", message);
    });

    println!("End the method...");

}

fn capture_mutable_reference() {
    let mut b1 = String::from("┌─────────────────┐");
    let mut b2 = String::from("└─────────────────┘");

    let mut display_heading = |s| {
        b1.push_str("🎃");
        b2.push_str("🎃");
        println!("{}", b1);
        println!("│ {:<15} │", s);
        println!("{}\n", b2);
    };

    display_heading(String::from("hello"));
    display_heading(String::from("goodbye!"));

    println!("b1: {}\nb2: {}\n", b1, b2);
}

fn capture_immutable_reference() {
    let b1 = String::from("┌─────────────────┐");
    let b2 = String::from("└─────────────────┘");

    let display_heading = |s| {
        println!("{}", b1);
        println!("│ {:<15} │", s);
        println!("{}\n", b2);
    };

    display_heading(String::from("hello"));
    display_heading(String::from("goodbye!"));

    println!("b1: {}\nb2: {}\n", b1, b2);
}

fn closure_multiple_statements_inferred_types() {
    let get_timestamp_after_delay = |seconds| {
        sleep(Duration::new(seconds, 0));
        Utc::now()
    };
    println!("Timestamp: {}", get_timestamp_after_delay(1).format("%T"));
}

fn closure_many_params_inferred_types() {
    let prod = |a, b| a * b;
    println!("Product: {}", prod(20, 5));
}

fn closure_one_param_inferred_types() {
    let reciprocal = |n| if n == 0.0 { 0.0 } else { 1.0 / n };
    println!("Reciprocal: {}", reciprocal(5.0));
    // println!("Reciprocal: {}", reciprocal(5));
}

fn closure_no_params_inferred_types() {
    let get_timestamp = || Utc::now();
    println!("Timestamp: {}", get_timestamp().format("%T"));
}

fn closure_multiple_statements() {
    let get_timestamp_after_delay = |seconds : u64| -> DateTime<Utc> {
        sleep(Duration::new(seconds, 0));
        Utc::now()
    };
    println!("Timestamp: {}", get_timestamp_after_delay(1).format("%T"));
}

fn closure_many_params() {
    let prod = |a: i32, b: i32| -> i32 { a * b};
    println!("Product: {}", prod(20, 5));
}

fn closure_one_param() {
    let reciprocal = |n: f64| -> f64 { if n == 0.0 { 0.0 } else { 1.0 / n } };
    println!("Reciprocal: {}", reciprocal(5.0));
}

fn closure_no_params() {
    let get_timestamp = || -> DateTime<Utc> { Utc::now() };
    println!("Timestamp: {}", get_timestamp());
}