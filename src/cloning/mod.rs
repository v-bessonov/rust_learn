pub fn demo_cloning(){
    println!("\nDemo cloning");

    let a = 42;
    let b = a;
    println!("a: {}, b: {}", a, b);

    let mut s1 = String::from("hello");
    let s2 = s1.clone();

    s1.push_str(", world");

    println!("s1: {}", s1);
    println!("s2: {}", s2);
}