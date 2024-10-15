pub enum Color {
    #[allow(dead_code)] Red,
    #[allow(dead_code)] Green,
    #[allow(dead_code)] Blue,
}

pub enum HouseLocation {
    #[allow(dead_code)] Number(i32),
    #[allow(dead_code)] Name(String),
    #[allow(dead_code)] Unknown,
}

pub fn demo_simple_enums() {
    println!("\nDemo simple enums");
    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue")
    }
}

pub fn demo_enum_with_data() {
    println!("\nDemo enum with data");

    let h: HouseLocation = HouseLocation::Number(4);
    match h {
        HouseLocation::Number(num) => println!("You live in a house number: {}", num),
        HouseLocation::Name(name) => println!("You live in a house named: {}", name),
        HouseLocation::Unknown => println!("Your house location is unknown")
    }

    let size = std::mem::size_of::<HouseLocation>();
    println!("Btw the size of HouseLocation is {} bytes", size);
}

pub fn demo_using_option_enum() {
    println!("\nDemo using the Option<T> enum");
    let sec = sec_of_day(23, 59, 59);

    match sec {
        Some(sec) => println!("Second of the day: {}", sec),
        None => println!("Second of the day: no value available")
    }

    println!("Unwrapped sec: {}", sec.unwrap_or(0));
}

fn sec_of_day(h: u32, m: u32, s: u32) -> Option<u32> {
    if h <= 23 && m <= 59 && s <= 59 {
        let sec = h * 3600 + m * 60 + s;
        return Option::Some(sec);
    }
    return Option::None;
}

pub fn demo_using_result_enum(){
    println!("\nDemo using the Result<T,E> enum");

    let res: Result<i32, std::num::ParseIntError>;

    res = i32::from_str_radix("FF", 16);

    match res {
        Ok(n) => println!("Parsed str as i32: {}", n),
        Err(err) => println!("Error occurred: {}", err),
    }

    let res2 = i32::from_str_radix("FF", 16);
    println!("Unwrapped result: {}", res2.unwrap_or(-1));
}