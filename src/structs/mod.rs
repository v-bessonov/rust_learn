struct Employee {
    pub name: String,
    pub salary: u64,
    pub full_time: bool
}

pub fn demo_accessing_struct() {
    println!("\nDemo accessing structs");

    let _e1: crate::mytypes::employee::Employee;

    let _e2: Employee;

    let size = std::mem::size_of::<Employee>();
    println!("The size of Employee is {} bytes", size);
}

pub fn demo_struct_instances() {
    println!("\nDemo instances");
    create_immutable_instance();
    create_mutable_instance();
}

pub fn demo_struct_pass_value() {
    println!("\nDemo structs pass value");
    let e1 = Employee {
        name: String::from("John"),
        salary: 1000,
        full_time: false,
    };

    consume_employee(e1);
    // Can't use e1 now
    // println!("{}", e1.name);
}

pub fn demo_struct_pass_reference() {
    println!("\nDemo structs pass reference");

    let mut e1 = Employee {
        name: String::from("Jane"),
        salary: 1000,
        full_time: true,
    };

    print_employee(&e1);
    reward_employee(&mut e1);
    print_employee(&e1);
}

pub fn demo_struct_return_value() {
    println!("\nDemo structs return value");

    let e1 = build_employee(String::from("Jane"), 1000, true);
    show_employee(&e1);

    let mut e2 = build_employee_v2(String::from("John"), 1000, false);
    e2.salary += 750;
    show_employee(&e2);
}

pub fn demo_struct_return_reference(){
    println!("\nDemo struct return reference");
    let mut e1 = build_employee(String::from("Jane"), 1001, true);
    let mut e2 = build_employee_v2(String::from("John"), 1000, false);

    let ri = choose_employee(&e1, &e2);
    show_employee(ri);

    let rm = choose_mut_employee(&mut e1, &mut e2);
    rm.salary *= 2;
    show_employee(rm);
}

fn choose_employee<'a>(e1 : &'a Employee, e2 : &'a Employee) -> &'a Employee {
    if e1.salary > e2.salary {e1} else {e2}
}

fn choose_mut_employee<'a>(e1 : &'a mut Employee, e2 : &'a mut Employee) -> &'a mut Employee {
    if e1.salary > e2.salary {e1} else {e2}
}

fn build_employee(name: String, salary: u64, full_time: bool) -> Employee {
    Employee { name: name, salary: salary, full_time: full_time }
}

fn build_employee_v2(name: String, salary: u64, full_time: bool) -> Employee {
    Employee { name, salary, full_time }
}

fn reward_employee(e: &mut Employee) {
    (*e).salary += 500;
    e.salary += 250;
}

fn show_employee(e: &Employee) {
    println!("{} earns {}, full_time status: {}", e.name, e.salary, e.full_time);
}

fn print_employee(e: &Employee) {
    println!("Using explicit dereferencing: {} earns {}, full_time status: {}", (*e).name, (*e).salary, (*e).full_time);
    println!("Using implicit dereferencing: {} earns {}, full_time status: {}", e.name, e.salary, e.full_time);
}

fn consume_employee(e: Employee) {
    println!("{} earns {}, full_time status: {}", e.name, e.salary, e.full_time);
    // Employee object dropped here
}

fn create_immutable_instance() {
    let e1 = Employee {
        name: String::from("Jane"),
        salary: 1000,
        full_time: true,
    };
    println!("{} earns {}, full_time status: {}", e1.name, e1.salary, e1.full_time);
}

fn create_mutable_instance() {
    let mut e2 = Employee {
        name: String::from("John"),
        salary: 1000,
        full_time: false,
    };

    e2.salary *= 2;

    println!("{} earns {}, full_time status: {}", e2.name, e2.salary, e2.full_time);
}