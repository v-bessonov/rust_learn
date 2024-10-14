pub fn demo_additional_techniques() {
    let a = -12345;
    let b = 3.14;
    let c = 'C';

    println!("\na is {}, b is {}, c is {}", a, b, c);

    let d = 0;

    println!("\nd is {}", d);

    let mut e = 0;
    println!("\ne originally is {}", e);
    e = 1;
    println!("\ne afterwards is {}", e);

    let _f = 0;

    let g = 3.99;
    let h = g as i32;

    println!("\ng is {}, h is {}", g, h);


    let num = "12345";
    println!("\nnum as a string is {}", num);

    let num = 12345;
    println!("\nnum+1 as a number is {}", num+1);

    const SECONDS_IN_HOUR: i32 = 3_600;
    const SECONDS_IN_DAY: i32 = 24 * SECONDS_IN_HOUR;

    println!("\nThere are {} seconds in a day", SECONDS_IN_DAY);
}