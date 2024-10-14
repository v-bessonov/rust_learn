pub fn demo_if() {
    let age = 58;

    if age > 50 {
        println!("You are old!");
    }

    let height = 1.67;
    if height > 1.8 {
        println!("You are tall!");
    } else {
        println!("You are not so tall!");
    }

    let swan_games = 500;
    if swan_games > 300 {
        println!("You are a very loyal fan, we appreciate it");
    } else if swan_games > 100 {
        println!("You are discerning fan");
    } else {
        println!("You are quite new fan, welcome");
    }

    let msg = if age > 50 { "old" } else { "young" };
    println!("You are {}", msg);
}

pub fn demo_match() {
    let num = 100;
    println!("\nUsing a match to test an expression against pattern");

    match num {
        100 => println!("A hundred"),
        200 => println!("Two hundred"),
        _ => println!("Something else")
    }

    match num {
        25..=50 => println!("25 to 50"),
        51..=100 => println!("51 to 100"),
        _ => println!("Something else")
    }

    match num {
        25 | 50 | 75 => println!("25, 50 or 75"),
        100 | 200 => println!("100 or 200"),
        _ => println!("Something else")
    }

    let res = match num {
        x if x < 50 => "Less than 50",
        x if x == 75 => "75",
        _ => "Something else"
    };
    println!("Result of match expression: {}", res);
}

pub fn demo_loops() {
    /*println!("\nUsing an infinite loops");
    loop {
        println!("This loop will go on forever! Press Ctrl-C to stop me!");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }*/

    println!("\nUsing a while loop");
    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }

    println!("\nUsing a for loop over an array");
    let arr = [99, 55, 95, 100, 82];
    for elem in arr {
        println!("{}", elem);
    }

    println!("\nUsing a for loop over a range (exclusive upper limit)");
    for i in 0..10 {
        println!("{}", i);
    }

    println!("\nUsing a for loop over a range (inclusive upper limit)");
    for i in 0..=10 {
        println!("{}", i);
    }
}

pub fn demo_break_continue() {
    println!("\nDemo using break and continue");
    let arr = [99, 45, 85, 100, 82];

    for elem in arr {
        if elem == 100 {
            println!("Found 100, so break out of loop completely");
            break;
        }
        println!("{}", elem);
    }

    for elem in arr {
        if elem < 50 {
            println!("Found value less than 50, continue to next iteration");
            continue;
        }
        println!("{}", elem);
    }

    'outer: loop {
        println!("Entered the outer loop");
        loop {
            println!("Entered the inner loop");
            break 'outer;
        }
    }
    println!("Exited the outer loop");
    println!("The end");
}