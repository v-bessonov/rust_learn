use std::collections::HashMap;

pub fn demo_maps(){
    println!("\nUsing maps");

    let mut m: HashMap<String, i32> = HashMap::new();
    let mut _m2 = HashMap::<String, i32>::new();

    m.insert(String::from("UK"), 44);
    m.insert(String::from("NO"), 47);
    m.insert(String::from("SG"), 65);

    m.entry(String::from("SA")).or_insert(27);

    let val = m["UK"];
    println!("Value: {}", val);

    let opt = m.get("UK");
    match opt {
        Some(value) => println!("Value: {}", value),
        None => println!("No value"),
    }

    println!("Entries in m:");
    for entry in &m {
        println!("    {:?}", entry);
    }

    println!("m is {:?}", m);

}