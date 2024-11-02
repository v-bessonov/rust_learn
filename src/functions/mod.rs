pub fn demo_passing_value() {
    println!("\nDemo passing value");

    let n = 42;
    let s = String::from("hello");

    some_func(n, s);

    println!("n = {}", n);
    // ownership moved
    // println!("s = {}", s);
}

pub fn demo_passing_references() {
    println!("\nDemo passing references");

    let n = 42;
    let s = String::from("Ola Nordmann");

    some_func1(&n, &s);
    // Can't pass &str to &String
    // some_func1(&n, "Jo");

    some_func2(&n, &s);
    some_func2(&n, "Siv Nordmann");

    some_func3(&n, &s);
    some_func3(&n, "Per Nordmann");

    println!("n = {}", n);
    println!("s = {}", s);
}

pub fn demo_passing_mutable_references() {
    println!("\nDemo passing mutable references");

    let mut n = 42;
    let mut s = String::from("hello");

    some_func100(&mut n, &mut s);

    n += 1_000_000;
    s.push_str(" 👍 👍 👍");

    println!("n: {}", n);
    println!("s: {}", s);
}

pub fn demo_returning_value() {
    println!("\nDemo returning value");

    let n = func_returning_copyable_type();
    println!("n = {}", n);

    let s = func_returning_non_copyable_type();
    println!("s = {}", s);
}

pub fn demo_returning_reference() {
    println!("\nDemo returning reference");

    let s = String::from("hello world");
    let r1 = get_first_word(&s);
    println!("r1 = {}", r1);

    let r2: &str = get_first_word(&s);
    println!("r2 = {}", r2);

    let message: &str = get_message(99);
    println!("message = {}", message);
}

pub fn demo_returning_mutable_reference() {
    println!("\n\nDemo returning mutable reference");
    let mut s = String::from("hello");
    let r = some_func200(&mut s);

    r.push_str(" and goodbye");
    println!("r: {}", r);
}

fn some_func200(s: &mut String) -> &mut String {
    s.push_str(" world");
    s
}

fn get_first_word(s: &str) -> &str {
    let mut pos = 0;
    for ch in s.chars() {
        if ch == ' ' {
            break;
        }
        pos += 1;
    }
    &s[..pos]
}

fn get_message(mark: i32) -> &'static str {
    if mark >= 50 { "PASS😂" } else { "FAIL😒" }
}

fn func_returning_non_copyable_type() -> String {
    let s = String::from("hello");
    return s;
    // Moves ownership back to caller
}

fn func_returning_copyable_type() -> i32 {
    let n = 42;
    return n;
    // Copies value back to caller
}

fn some_func100(iparam: &mut i32, sparam: &mut String) {
    println!("Values initially: {}, {}", iparam, sparam);
    *iparam += 10;
    sparam.push_str(" world");
    println!("Values aferward: {}, {}", iparam, sparam);
}

fn some_func1(iparam: &i32, sparam: &String) {
    if *iparam >= 50 {
        println!("{}, {}, PASS 😂", *iparam, (*sparam).to_uppercase());
    } else {
        println!("{}, {}, FAIL 😂", *iparam, (*sparam).to_uppercase());
    }

    // Both the same
    // (*sparam).to_uppercase()
    // sparam.to_uppercase()

}

fn some_func2(iparam: &i32, sparam: &str) {
    if *iparam >= 50 {
        println!("{}, {}, PASS 😂", *iparam, sparam.to_uppercase());
    } else {
        println!("{}, {}, FAIL 😂", *iparam, sparam.to_uppercase());
    }
}

fn some_func3(iparam: &i32, sparam: &str) {
    println!("Values {0} and {1}, addresses {0:p} and {1:p}", iparam, sparam);
}

fn some_func(iparam: i32, sparam: String) {
    println!("In some_func, iparam is {}", iparam);
    println!("In some_func, sparam is {}", sparam);
}