pub fn demo_string_handling(){
    println!("\nDemo string handling");
    using_string_literals();
    using_string_objects();
    using_mutable_string_objects();
}

pub fn demo_string_slice_intro(){
    println!("\nDemo string slice intro");
    slice_string_object();
    slice_string_literal();
}

pub fn demo_string_slice_techniques(){
    println!("\nDemo string slice techniques");
    slice_usage();
    slice_iteration();
    slice_part_of_string();
    slice_mutability();
}

fn slice_mutability() {
    let mut message = String::from("hello ");
    message.push_str(", world!");

    if true {
        let s : &mut str = &mut message[9..];
        s.make_ascii_uppercase();
    }

    println!("{}", message);
}

fn slice_part_of_string() {
    let message = "howdy😂";

    let s3 = &message[0..3];
    let s4 = &message[..3];
    let s5 = &message[2..5];
    let s6 = &message[2..];

    println!("\ns3 ptr: {:p}, len: {}, text: {}", s3.as_ptr(), s3.len(), s3);
    println!("s4 ptr: {:p}, len: {}, text: {}", s4.as_ptr(), s4.len(), s4);
    println!("s5 ptr: {:p}, len: {}, text: {}", s5.as_ptr(), s5.len(), s5);
    println!("s6 ptr: {:p}, len: {}, text: {}", s6.as_ptr(), s6.len(), s6);
}

fn slice_iteration() {
    let s2 = "Hello😂";
    //let s2 = String::from("World😂");

    println!("\nRaw bytes in s2 (in decimal, hex and octal)");

    for b in s2.bytes(){
        println!("   {}, {:x}, {:o}", b, b, b);
    }

    println!("\nCharacters in s2");

    for ch in s2.chars(){
        println!("   {}", ch);
    }

}

fn slice_usage() {
    let s1 = "Hello😂";
    //let s1 = String::from("World😂");

    println!("s1 ptr: {:p}, len: {}, text: {}", s1.as_ptr(), s1.len(), s1);
}

fn slice_string_literal() {
    let s3 = "hello";

    let s4: &'static str = "world";
    println!("s3: {}, s4: {}", s3, s4);
}

fn slice_string_object() {
    let obj = String::from("hello");

    let s1 = &obj;
    let s2: &str = &obj;

    println!("s1: {}, s2: {}", s1, s2);
}

fn using_mutable_string_objects() {
    let mut s5 = String::from("   super");
    s5.push_str("   bowl!   ");

    let s6 = s5.trim().to_uppercase();

    println!("s5: {}, ptr: {:p}, len: {}", s5, s5.as_ptr(), s5.len());
    println!("s6: {}, ptr: {:p}, len: {}", s6, s6.as_ptr(), s6.len());
}

fn using_string_objects() {
    let s3 = String::from("wales");
    let s4: String = String::from("scotland");

    println!("s3: {}, ptr: {:p}, len: {}", s3, s3.as_ptr(), s3.len());
    println!("s4: {}, ptr: {:p}, len: {}", s4, s4.as_ptr(), s4.len());
}

fn using_string_literals() {
    let s1 = "hello";
    let s2: &'static str = "world";
    println!("s1: {}, ptr: {:p}, len: {}", s1, s1.as_ptr(), s1.len());
    println!("s2: {}, ptr: {:p}, len: {}", s2, s2.as_ptr(), s2.len());
}