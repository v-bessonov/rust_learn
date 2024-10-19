use std::sync::atomic::{AtomicI32, Ordering};
use std::thread::sleep;
use std::time::Duration;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;

pub static GLOBAL_MESSAGE: &str = "This is a global message";

static GLOBAL_TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
    let now = Utc::now();
    println!("global GLOBAL_TIMESTAMP: {} ***** initialization *****", now.format("%T"));
    return now;
});

static GLOBAL_COUNT: AtomicI32 = AtomicI32::new(0);

pub fn demo_locals() {
    println!("\nDemo scope local");

    let x = 42;

    if x != 0 {
        let s1 = "Andy";
        println!("s1: {}", s1);
    }
}

pub fn demo_static_local() {
    println!("\nDemo scope static local");

    static_init_at_compile_time();
    static_init_at_run_time();
}

pub fn demo_static_global() {
    println!("\nDemo scope static global");

    f1();
    f1();
    f2();
    f2();
}

pub fn demo_static_mutable(){
    println!("\nDemo scope static mutable");
    unsafe{
        f3();
        f3();
    }

    f4();
    f4();
}

fn f4() {
    static LOCAL_COUNT: AtomicI32 = AtomicI32::new(0);
    let mut x = 0;

    LOCAL_COUNT.fetch_add(1, Ordering::Relaxed);
    x += 1;

    GLOBAL_COUNT.fetch_add(1, Ordering::Relaxed);

    println!("f4, LOCAL_COUNT: {:?}, x: {}, GLOBAL_COUNT: {:?}", LOCAL_COUNT, x, GLOBAL_COUNT);
}

unsafe fn f3() {
    static mut LOCAL_COUNT: i32 = 0;
    let mut x = 0;
    LOCAL_COUNT +=1;
    x += 1;

    GLOBAL_COUNT.fetch_add(1, Ordering::Relaxed);

    println!("f3, LOCAL_COUNT: {}, x: {}, GLOBAL_COUNT: {:?}", LOCAL_COUNT, x, GLOBAL_COUNT);
}

fn f2() {
    println!("f2, GLOBAL MESSAGE: {}", GLOBAL_MESSAGE);
    println!("f2, GLOBAL TIMESTAMP: {}", (*GLOBAL_TIMESTAMP).format("%T"));
}

fn f1() {
    println!("f1, GLOBAL MESSAGE: {}", GLOBAL_MESSAGE);
    println!("f1, GLOBAL TIMESTAMP: {}", (*GLOBAL_TIMESTAMP).format("%T"));
}

fn static_init_at_compile_time() {
    static MESSAGE: &str = "Scope static compile time";
    println!("MESSAGE: {}", MESSAGE);
}

fn static_init_at_run_time() {
    println!("Current time: {}", Utc::now().format("%T"));
    static TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
        sleep(Duration::new(5, 0));
        let now = Utc::now();
        println!("Current time: {}", now.format("%T"));
        return now;
    });

    println!("TIMESTAMP time: {}", (*TIMESTAMP).format("%T"));
}