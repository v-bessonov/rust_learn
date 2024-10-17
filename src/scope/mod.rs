pub fn demo_locals() {
    println!("\nDemo scope local");

    let x = 42;

    if x != 0 {
        let s1 = "Andy";
        println!("s1: {}", s1);
    }
}