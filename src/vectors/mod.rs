pub fn demo_vectors() {
    println!("\nUsing vectors");

    let mut _v1: Vec<i32> = Vec::new();
    let mut _v2: Vec<i32> = Vec::<i32>::new();

    let mut v3 = vec![100, 101, 102];

    let item = v3[0];
    println!("Value: {}", item);

    let opt = v3.get(0);

    match opt {
        Some(value) => println!("Value: {}", value),
        None => println!("No value"),
    }
    v3.push(103);
    v3.push(104);
    v3.push(105);
    v3.pop();
    v3.insert(0, 99);

    println!("Items in v3:");
    for item in &v3 {
        println!("   {}", item);
    }

    println!("v3 is {:?}", v3);
}