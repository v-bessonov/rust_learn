pub fn demo_simple_borrowing(){
    println!("\nDemo simple borrowing");
    implicitly_typed_borrowing();
    explicitly_typed_borrowing();
    mutable_borrowing();
}

pub fn demo_borrow_checker(){
    println!("\nDemo borrow checker");
    defining_many_immutable_references();
    restrictions_after_defining_mutable_reference();
    restrictions_after_defining_immutable_reference();
}

fn restrictions_after_defining_immutable_reference() {
    let mut s = String::from("Hello");
    s.push_str(", World!");

    let r1 = &s;
    let r2 = &s;

    // Can't do this
    //let r3 = &mut s;
    //s.push_str(", World!");

    println!("s: {}, r1: {}, r2: {}", s, r1, r2);
}

fn restrictions_after_defining_mutable_reference() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    let r1 = &mut s;
    r1.push_str(", world!");

    // Can't have any other borrows
    //let r2 = &mut s;
    //let r3 = &s;
    //println!("s: {}", s);

    r1.push_str(", world!");

    println!("r1: {}", r1);
}

fn defining_many_immutable_references() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
}

fn mutable_borrowing() {
    let mut s3 : String = String::from("hello");
    let r3 : &mut String = &mut s3;

    r3.push_str(", world");

    println!("Mutable borrowing, r3: {}", r3);
}

fn explicitly_typed_borrowing() {
    let s2: String = String::from("world");
    let r2 : &String = &s2;

    println!("Explicitly typed borrowing, s2:{}, r2: {}", s2, r2);
}

fn implicitly_typed_borrowing() {
    let s1 = String::from("hello");
    let r1 = &s1;

    println!("Implicitly typed borrowing, s1:{}, r1: {}", s1, r1);
}