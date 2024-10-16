pub fn demo_tuples(){
    println!("\nUsing tuples");

    let t1 = (9, "hi", 3.5);
    println!("t1 elements are {}, {}, {}", t1.0, t1.1, t1.2);

    let mut t2 = (9, "hi", 3.5);
    t2.0 = 99;
    println!("t2 elements are {}, {}, {}", t2.0, t2.0, t2.1);

    let t3 = ();
    println!("t3 is {:?}", t3);

    let t4: (i32, bool, f64);
    t4 = (58, true, 1.67);
    println!("t4 is {:?}, elements are: {}, {}, {}", t4, t4.0, t4.1, t4.2);
}