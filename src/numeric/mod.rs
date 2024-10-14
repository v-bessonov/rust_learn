pub fn demo_integers() {
    let a1: i32 = -12345;
    let a2: i32 = 0xFFFF;
    let a3: i32 = 0o177;
    let a4: i32 = 0b1110;

    let b: u32 = 12345;

    let c: isize = 24680;

    println!("\nNumbers are {} {} {} {}, {}, {}", a1, a2, a3, a4, c, b);
    println!("Numbers in reverse order are {5} {4} {3} {2} {1} {0}", a1, a2, a3, a4, c, b);
    println!("isize is {} bytes on my machine", size_of::<isize>());
}

pub fn demo_floats() {
    let f1: f32 = 1.23456;
    let f2: f64 = 9.87654;

    println!("\nFloats are {} {}", f1, f2);
    println!("Floats to 2dp are: {:.2} {:.2}", f1, f2);
    println!("Floats field width 10 L-aligned filled with space are : ***{:<10.2}*** and ***{:<10.2}***", f1, f2);
    println!("Floats field width 10 R-aligned filled with space are : ***{:>10.2}*** and ***{:>10.2}***", f1, f2);
    println!("Floats field width 10 L-aligned filled with tilde are : ***{:~<10.2}*** and ***{:~<10.2}***", f1, f2);
    println!("Floats field width 10 R-aligned filled with tilde are : ***{:~>10.2}*** and ***{:~>10.2}***", f1, f2);


    let f3: f32 = -1.60217663e-16;
    let f4: f64 = 2.99792458e8;

    println!("\nElectron charge: {0}, {0:e}, {0:.4e}", f3);
    println!("Speed of light: {0}, {0:e}, {0:.4e}", f4);
}