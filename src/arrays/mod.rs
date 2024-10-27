pub fn demo_arrays() {
    println!("\nUsing arrays");

    let a1 = [100, 101, 102];
    println!("a1 length is {}, first element is {}", a1.len(), a1[0]);

    let mut a2 = [100, 101, 102];
    a2[0]= 999;
    println!("a2 length is {}, first element is {}", a2.len(), a2[0]);

    println!("Elements in a2");
    for elem in a2 {
        print!("{} ", elem);
    }
}

pub fn demo_arrays_techniques() {
    println!("\nArray techniques");

    let a1: [i64; 5];
    a1 = [100, 101, 102, 103, 104];
    println!("a1 is {:?}", a1);

    let mut a2 = [99; 5];
    a2[0] = 58;
    a2[4] = 25;
    println!("a2 is {:?}", a2);
}

pub fn demo_array_slice_intro (){
    println!("\nDemo array slice intro");

    let a = [10, 11, 12, 13, 14];

    let s1 = &a;

    let s2: &[i32] = &a;

    println!("s1 ptr: {:p}, len: {}, elements: {:?}", s1.as_ptr(), s1.len(), s1);
    println!("s2 ptr: {:p}, len: {}, elements: {:?}", s2.as_ptr(), s2.len(), s2);
}

pub fn demo_array_slice_techniques (){
    println!("\nDemo array slice techniques");

    slice_iteration();
    slice_part_of_array();
    slice_mutability();
}

fn slice_mutability() {
    let mut a = [10, 11, 12, 13, 14];
    a[0] = 100;

    if  true {
        let s: &mut [i32]  = &mut a[2..4];
        s[0] = 130;
    }
    println!("a: {:?}", a);
}

fn slice_part_of_array() {
    let a = [10, 11, 12, 13, 14];

    let s2 = &a[0..3];
    let s3 = &a[..3];
    let s4 = &a[2..4];
    let s5 = &a[2..];

    println!("\ns2 ptr: {:p}, len: {}, elements: {:?}", s2.as_ptr(), s2.len(), s2);
    println!("s3 ptr: {:p}, len: {}, elements: {:?}", s3.as_ptr(), s3.len(), s3);
    println!("s4 ptr: {:p}, len: {}, elements: {:?}", s4.as_ptr(), s4.len(), s4);
    println!("s5 ptr: {:p}, len: {}, elements: {:?}", s5.as_ptr(), s5.len(), s5);
}

fn slice_iteration() {
    let a = [10, 11, 12, 13, 14];

    let s1 = &a;

    println!("\nElements in s1:");

    for elem in s1 {
        println!("   {} ", elem);
    }
}