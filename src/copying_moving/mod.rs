pub fn demo_copying_moving(){
    println!("\nDemo copying moving");
    let a = 42;
    let b = a;
    println!("a: {}, b: {}", a, b);

    let s1 = String::from("hello");
    let s2 = s1;

    // Compiler error here
    // println!("s1: {}", s1);
    println!("s2: {}", s2);
}