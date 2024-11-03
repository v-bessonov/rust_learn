use crate::mytypes::employee::Employee;
use crate::mytypes::point3d::Point3D;
use crate::mytypes::point::Point;

pub fn demo_simple_struct_implementation() {
    println!("\nDemo simple struct implementation");

    let p1 = Point { x: 10, y: 20 };

    p1.print();
    p1.print_v1();
    p1.print_v2();
    p1.print_v3();

    println!("{}", p1.to_string());
}

pub fn demo_mutable_struct_implementation() {
    println!("\nDemo mutable struct implementation");
    let mut p1 = Point { x: 10, y: 20 };

    p1.move_by(100, 200);
    println!("{}", p1.to_string());

    p1.reset_v1();
    p1.reset_v2();
    p1.reset_v3();

    println!("{}", p1.to_string());
}

pub fn demo_modular_code_struct_implementation() {
    println!("\nDemo modular code struct implementation");

    let mut p1 = Point { x: 10, y: 20 };

    p1.move_by(100, 200);
    println!("{}", p1.to_string());

    p1.reset();
    println!("{}", p1.to_string());
}

pub fn demo_associated_functions_struct_implementation() {
    println!("\nDemo associated functions struct implementation");
    let mut p1 = Point3D::new(10, 20, 30);

    p1.move_by(100, 200, 300);
    println!("{}", p1.to_string());

    p1.reset();
    p1.print();
    println!("{}", p1.to_string());

}

pub fn demo_associated_data_struct_implementation() {
    println!("\nDemo associated data struct implementation");

    let e1 = Employee::new(String::from("Matt"), 10_000, true);
    println!("{}", e1.to_string());

    let e2 = Employee::new(String::from("Mark"), 20_000, true);
    println!("{}", e2.to_string());

    let e3 = Employee::new(String::from("Luke"), 30_000, true);
    println!("{}", e3.to_string());

    let mut e4 = Employee::new(String::from("John"), 40_000, true);
    e4.payrise(500_000);
    println!("{}", e4.to_string());

}