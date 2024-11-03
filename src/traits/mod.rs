use crate::mystructs::dequeimp::MyDeque;
use crate::mystructs::{debuggable, displayable};
use crate::mystructs::cloneable::Flight;
use crate::mystructs::copyable::Currency;
use crate::mystructs::droppable::Place;
use crate::mystructs::employee::Employee;
use crate::mystructs::point::Point;
use crate::mytraits::collections::{Deque, Queue};
use crate::mytraits::log::Log;
use crate::mytraits::print::Print;

pub fn demo_trait_essentials() {
    println!("\nDemo trait essentials");

    let mut e1 = Employee::new(String::from("John"), 100, false);
    e1.payrise(100);

    e1.print();
}

pub fn demo_trait_techniques() {
    println!("\nDemo trait techniques");
    let mut e1 = Employee::new(String::from("Mary"), 100, false);

    e1.payrise(100);
    e1.log();
    e1.log_verbose();
    println!("BTW Employee::LOG_TIMESTAMP is {}", Employee::LOG_TIMESTAMP);
}

pub fn demo_inheritance_polymorphism() {
    println!("\nDemo inheritance polymorphism");

    let obj1 = Point::new(10, 20);
    let obj2 = Employee::new(String::from("Tom"), 1000, false);

    println!("Printable things:");
    print_something(&obj1);
    print_something(&obj2);

    let vec: Vec::<&dyn Print> = vec![&obj1, &obj2];
    println!("Print things in a polymorphic collection:");
    for obj in vec {
        obj.print();
    }

}

pub fn demo_trait_inheritance() {
    println!("\nDemo trait inheritance");

    let mut d = MyDeque::new();


    d.push_back(300);
    d.push_back(400);
    d.push_back(500);
    d.push_front(200);
    d.push_front(100);
    d.push_front(0);
    d.pop_front();

    println!("MyDeque object has {} items:", d.len());

    loop {
        match d.pop_back() {
            Some(v) => println!("  {}", v),
            None => break
        }
    }
}

pub fn demo_displayable(){
    println!("\nDemo displayable");

    let c = displayable::Coord::new(51.62, 3.94);
    println!("{}", c);
}

pub fn demo_debuggable() {
    println!("\nDemo debuggable");
    let c = debuggable::Coord::new(51.62, 3.94);
    println!("{:?}", c);
}

pub fn demo_droppable(){
    println!("\nDemo droppable");

    let _p1 = Place::new("Dresden", "Germany");

    if true {
        let _p2 = Place::new("Paris", "France");
    }

    println!("Goodbye");
}

pub fn demo_cloneable(){
    println!("\nDemo cloneable");

    let f1 = Flight::new("BRL", "IST");
    let mut f2 = f1.clone();

    f2.redirect_to("DUB");

    println!("f1: {:?}", f1);
    println!("f2: {:?}", f2);
}

pub fn demo_copyable(){
    println!("\nDemo copyable");

    let mut c1 = Currency::new(10, 99);
    let mut c2 = c1;

    c1.dollars = 11;

    c2.dollars = 22;

    println!("c1: {:?}", c1);
    println!("c2: {:?}", c2);

}

fn print_something(p: &dyn Print) {
    p.print();
}