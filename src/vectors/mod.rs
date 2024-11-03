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

pub fn demo_closures_iteration() {
    println!("\nDemo closures iteration");
    demo_simple_iteration();
    demo_unused_closure_variable();
    demo_filtering_mapping();
    demo_collecting_result();
}

fn demo_filtering_mapping() {
    let v = vec!["donald", "huey", "louie", "dewey"];
    println!("Uppercase 'd' ducks:");

    v.iter().filter(|e| e.starts_with("d"))
        .map(|e| e.to_uppercase())
        .for_each(|e| println!("    {}", e));
}

fn demo_collecting_result() {
    let v = vec!["donald", "huey", "louie", "dewey"];
    println!("Uppercase 'd' ducks:");

    let upper_y_ducks = v.iter().filter(|e| e.ends_with("y"))
        .map(|e| e.to_uppercase())
        .collect::<Vec<String>>();

    println!("There are {} ducks ending with 'y':", upper_y_ducks.len());
    upper_y_ducks.iter().for_each(|e| println!("    {}", e));
}

fn demo_unused_closure_variable() {
    let v = vec!["donald", "huey", "louie", "dewey"];
    println!("Redacted ducks:");

    v.iter().for_each(|_| println!("    xxx"));
}

fn demo_simple_iteration() {
    let v = vec!["donald", "huey", "louie", "dewey"];
    println!("All ducks:");

    v.iter().for_each(|e| println!("    {}", e));
}