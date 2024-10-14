pub fn demo_other_simple_types() {
    let is_welsh: bool = true;
    let can_sing: bool = false;

    println!("\nIs Welsh? {} Can sing? {}", is_welsh, can_sing);

    let is_welsh_num: i32 = is_welsh as i32;
    let can_sing_num: i32 = can_sing as i32;

    println!("Is Welsh as number: {} Can sing as number: {}", is_welsh_num, can_sing_num);

    let res1: bool = is_welsh && can_sing;
    let res2: bool = is_welsh || can_sing;
    let res3: bool = !(is_welsh || can_sing);

    println!("res1: {}, res2: {}, res3: {}", res1, res2, res3);

    let middle_initial: char = 'C';
    let favourite_emoji: char = 'φ';

    println!("\nMiddle initial: {}, fav emoji: {}", middle_initial, favourite_emoji);
}