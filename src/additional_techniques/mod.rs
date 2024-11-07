use std::rc::Rc;

extern "C" {
    fn abs(n: i32) -> i32;
    fn fabs(n: f64) -> f64;
}
struct Employee {
    name: String,
    salary: u64,
}

struct Node {
    data : i32,
    next : Option<Box<Node>>
}

impl Node {
    fn new(data : i32) -> Node {
        Node {data, next: None}
    }

    fn append(&mut self, data : i32) {
        match self.next {
            None => {
                self.next = Some(Box::new(Node::new(data)));
            }
            Some(ref mut boxed_next_node) => {
                boxed_next_node.append(data);
            }
        }
    }
    
    fn display(&self) {
        println!("{} ", self.data);
        match self.next {
            None => {
                println!("[END]");
            }
            Some(ref boxed_next_node) => {
                boxed_next_node.display();
            }
        }
    }
}

struct Chain {
    head : Option<Box<Node>>
}

impl Chain {
    fn new() -> Chain {
        Chain {head: None}
    }

    fn insert(&mut self, data : i32) {
        match self.head {
            None => {
                self.head = Some(Box::new(Node::new(data)));
            }
            Some(ref mut boxed_head_node) => {
                boxed_head_node.append(data);
            }
        }
    }

    fn display(&self) {
        match self.head {
            None => {
                println!("Empty chain!");
            }
            Some(ref boxed_head_node) => {
                boxed_head_node.display();
            }
        }
    }
}

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
    println!("\nnum+1 as a number is {}", num + 1);

    const SECONDS_IN_HOUR: i32 = 3_600;
    const SECONDS_IN_DAY: i32 = 24 * SECONDS_IN_HOUR;

    println!("\nThere are {} seconds in a day", SECONDS_IN_DAY);
}

pub fn demo_box1() {
    println!("\nDemo box1");

    let boxed_number = Box::new(42);
    println!("Explicitly dereferenced value: {}", *boxed_number);
    println!("Implicitly dereferenced value: {}", boxed_number);

    let value: i32 = *boxed_number;
    println!("Value: {}", value);
}

pub fn demo_box2() {
    println!("\nDemo box2");

    let boxed_employee = Box::new(Employee {name : String::from("Tom"), salary: 1000});

    process_employee(boxed_employee);

    // Can't use Box, ownership lost
    // println!("{} earns {}", boxed_employee.name, boxed_employee.salary);
}

pub fn demo_box3() {
    println!("\nDemo box3");

    let mut chain = Chain::new();
    chain.insert(100);
    chain.insert(200);
    chain.insert(300);
    chain.display();
}

pub fn demo_rc(){
    println!("\nDemo rc");

    let a = Rc::new(Employee {name : String::from("Emily"), salary: 1000});
    println!("Reference count initially is: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("Reference count after clone is: {}", Rc::strong_count(&b));

    use_employee(&a);
    println!("Reference count after function is: {}", Rc::strong_count(&a));

    if true {
        let d = Rc::clone(&a);
        println!("Reference count inside block is: {}", Rc::strong_count(&d));
    }

    println!("Reference count after block is: {}", Rc::strong_count(&a));

}

pub fn demo_unsafe_code(){
    println!("\nDemo unsafe code");

    let mut x = 100;
    x += 1;

    let mut y = 200;
    y +=1;

    // designates a raw pointer that treat the data as a constant
    let p1 : *const i32 = &x;

    // designates a raw pointer that treat the data as a mutable
    let p2: *mut i32 = &mut y;

    unsafe {
        println!("*p1 is: {}", *p1);

        // This won't compile because p1 treats data as a constant
        // *p1 = 111;
        // println!("*p1 is: {}", *p1);

        *p2 = 222;
        println!("*p2 is: {}", *p2);
    }

    println!("y: {}", y);
}

pub fn demo_language_integration(){
    println!("\nDemo language integration");

    unsafe {
        let res1 = abs(-42);
        println!("res1: {}", res1);

        let res2 = fabs(-3.14);
        println!("res2: {}", res2);

        let res3 = my_unsafe_function();
        println!("res3: {}", res3);
    }

    you_can_call_me_from_c();
}

unsafe fn my_unsafe_function() -> i32 {
    42
}

#[no_mangle]
pub extern "C"
fn you_can_call_me_from_c(){
    println!("Greetings from my Rust function.");
}

fn use_employee(rc_employee: &Rc<Employee>) {
    let c = Rc::clone(&rc_employee);
    println!("Reference count inside function is: {}", Rc::strong_count(&c));
}

fn process_employee(employee: Box<Employee>) {
    println!("{} earns {}", employee.name, employee.salary);
    println!("{} earns {}", (*employee).name, (*employee).salary);

} // Box implements the Drop trait and fn drop() called here. It deallocates the item on heap