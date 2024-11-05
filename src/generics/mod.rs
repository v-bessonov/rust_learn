#![allow(unused_variables, dead_code)]

use std::cmp::Ordering;
use std::collections::{HashMap};

struct Coordinate<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug)]
struct Wibble {}

struct PointV1 {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
struct PointV2 {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
struct TimeSeconds {
    s: i32,
}

#[derive(PartialEq)]
struct TimeMinutes {
    m: i32,
}

impl PartialEq<TimeMinutes> for TimeSeconds {
    fn eq(&self, other: &TimeMinutes) -> bool {
        self.s == other.m * 60
    }
}

impl PartialEq<TimeSeconds> for TimeMinutes {
    fn eq(&self, other: &TimeSeconds) -> bool {
        other == self
    }
}

struct Angle {
    degrees: i32,
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let d1 = self.degrees % 360;
        let d2 = other.degrees % 360;
        Some(d1.cmp(&d2))
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        self.degrees % 360 == other.degrees % 360
    }
}

#[derive(Eq, Hash, PartialEq)]
struct EmpCode {
    country: String,
    empnum: String,
}

impl EmpCode {
    fn new(country: &str, empnum: &str) -> EmpCode {
        EmpCode {
            country: country.to_string(),
            empnum: empnum.to_string(),
        }
    }
}

#[derive(Debug)]
struct Emp {
    name: String,
    salary: f32,
}

impl Emp {
    fn new(name: &str, salary: f32) -> Emp {
        Emp {
            name: name.to_string(),
            salary,
        }
    }
}

#[derive(PartialOrd, PartialEq)]
struct Currency {
    dollars: i32,
    cents: i32,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct ExamMark {
    value: i32,
}

pub fn demo_generic_structs() {
    println!("\nDemo generic structs");

    let c1 = Coordinate::<i32> { x: 1, y: 2, z: 3 };
    println!("{} {} {}", c1.x, c1.y, c1.z);

    let c2 = Coordinate { x: 1.1, y: 2.2, z: 3.3 };
    println!("{} {} {}", c2.x, c2.y, c2.z);
}

pub fn demo_generic_functions() {
    println!("\nDemo generic functions");

    let ai = [10, 20, 30, 40, 50];
    process_array_ints(&ai);

    let af = [10.1, 20.2, 30.3];
    process_array_floats(&af);

    process_array::<i32>(&ai);
    process_array::<f64>(&af);

    process_array(&ai);
    process_array(&af);
}

pub fn demo_type_constraints() {
    println!("\nDemo type constraints");
    let ai = [10, 20, 30, 40, 50];
    let af = [10.1, 20.2, 30.3];
    let aw = [Wibble {}, Wibble {}, Wibble {}];

    display_array(&ai);
    display_array(&af);
    display_array(&aw);
}

pub fn demo_partialeq_derived() {
    println!("\nDemo partialeq derived");

    demo_no_partial_equality();
    demo_partial_equality();
}

pub fn demo_partialeq_implemented() {
    println!("\nDemo partialeq implemented");

    let a1 = Angle { degrees: 90 };
    let a2 = Angle { degrees: 450 };
    let a3 = Angle { degrees: 180 };

    println!("a1 == a2   {}", a1 == a2);
    println!("a1.eq(&a2) {}", a1.eq(&a2));
    println!("a1 != a3   {}", a1 != a3);
    println!("a1.ne(&a3) {}", a1.ne(&a3));
}

pub fn demo_partialeq_implemented_diff_types() {
    println!("\nDemo partialeq implemented different types");

    let s1 = TimeSeconds { s: 300 };
    let s2 = TimeSeconds { s: 300 };
    let s3 = TimeSeconds { s: 1234 };

    let m1 = TimeMinutes { m: 5 };
    let m2 = TimeMinutes { m: 5 };
    let m3 = TimeMinutes { m: 5678 };

    println!("\nTimeSeconds == TimeSeconds etc.");
    println!("s1 == s2   {}", s1 == s2);
    println!("s1.eq(&s2) {}", s1.eq(&s2));
    println!("s1 != s3   {}", s1 != s3);
    println!("s1.ne(&s3) {}", s1.ne(&s3));

    println!("\nTimeMinutes == TimeMinutes etc.");
    println!("m1 == m2   {}", m1 == m2);
    println!("m1.eq(&m2) {}", m1.eq(&m2));
    println!("m1 != m3   {}", m1 != m3);
    println!("m1.ne(&m3) {}", m1.ne(&m3));

    println!("\nTimeSeconds == TimeMinutes etc.");
    println!("s1 == m1   {}", s1 == m1);
    println!("s1.eq(&m1) {}", s1.eq(&m1));
    println!("s1 != m3   {}", s1 != m3);
    println!("s1.ne(&m3) {}", s1.ne(&m3));

    println!("\nTimeMinutes == TimeSeconds etc.");
    println!("m1 == s1   {}", m1 == s1);
    println!("m1.eq(&s1) {}", m1.eq(&s1));
    println!("m1 != s3   {}", m1 != s3);
    println!("m1.ne(&s3) {}", m1.ne(&s3));
}

pub fn demo_eq_hash() {
    println!("\nDemo eq hash");

    let mut staff: HashMap<EmpCode, Emp> = HashMap::new();

    staff.insert(
        EmpCode::new("UK", "001"),
        Emp::new("Matt", 1000.0),
    );

    staff.insert(
        EmpCode::new("UK", "002"),
        Emp::new("Mark", 2000.0),
    );

    staff.insert(
        EmpCode::new("US", "001"),
        Emp::new("Mary", 3000.0),
    );

    let emp = &staff[&EmpCode::new("UK", "002")];
    println!("{:?}", emp);
}

pub fn demo_partial_ord() {
    println!("\nDemo partial ord");

    demo_partial_ord_derived();
    demo_partial_ord_implemented();
}

pub fn demo_ord() {
    println!("\nDemo ord");

    let m1 = ExamMark { value: 90 };
    let m2 = ExamMark { value: 99 };
    let m3 = ExamMark { value: 180 };
    let m4 = ExamMark { value: -42 };

    println!("m1.min(m2) {:?}", m1.min(m2));
    println!("m1.max(m2) {:?}", m1.max(m2));
    println!("m3 clamped {:?}", m3.clamp(ExamMark { value: 0 }, ExamMark { value: 100 }));
    println!("m4 clamped {:?}", m4.clamp(ExamMark { value: 0 }, ExamMark { value: 100 }));
}

pub fn demo_closures_fn_once() {
    println!("\nDemo closures fn once");

    // FnOnce
    let s1 = String::from("aaa");
    receive_fn_once(|| {
        println!("{}", s1);
        std::mem::drop(s1);
    });

    // FnMut
    let mut s2 = String::from("bbb");
    receive_fn_once(|| {
        s2.push_str("   BBB");
        println!("{}", s2);
    });

    // Fn
    let s3 = String::from("ccc");
    receive_fn_once(|| {
        println!("{}", s3);
    });
}

pub fn demo_closures_fn_mut() {
    println!("\nDemo closures fn mut");

    // Can't pass FnOnce
    // let s1 = String::from("aaa");
    // receive_fn_mut(|| {
    //     println!("{}", s1);
    //     std::mem::drop(s1);
    // });

    // FnMut
    let mut s2 = String::from("bbb");
    receive_fn_mut(|| {
        s2.push_str("   BBB");
        println!("{}", s2);
    });

    // Fn
    let s3 = String::from("ccc");
    receive_fn_mut(|| {
        println!("{}", s3);
    });
}

pub fn demo_closures_fn() {
    println!("\nDemo closures fn");

    // Can't pass FnOnce
    // let s1 = String::from("aaa");
    // receive_fn(|| {
    //     println!("{}", s1);
    //     std::mem::drop(s1);
    // });

    // Can't pass FnMut
    // let mut s2 = String::from("bbb");
    // receive_fn(|| {
    //     s2.push_str("   BBB");
    //     println!("{}", s2);
    // });

    // Fn
    let s3 = String::from("ccc");
    receive_fn(|| {
        println!("{}", s3);
    });
}

fn receive_fn<F>(func: F)
where
    F: Fn(),
{
    func();
    func();
}

fn receive_fn_mut<F>(mut func: F)
where
    F: FnMut(),
{
    func();
    func();
}

fn receive_fn_once<F>(func: F)
where
    F: FnOnce(),
{
    func();
}

fn demo_partial_ord_derived() {
    println!("\nDerived PartialOrd");

    let c1 = Currency { dollars: 10, cents: 75 };
    let c2 = Currency { dollars: 20, cents: 50 };
    let c3 = Currency { dollars: 20, cents: 75 };

    println!("c1 < c2    {}", c1 < c2);
    println!("c1.lt(&c2) {}", c1.lt(&c2));
    println!("c1 <= c2   {}", c1 <= c2);
    println!("c1.le(&c2) {}", c1.le(&c2));

    println!("c1 > c2    {}", c1 > c2);
    println!("c1.gt(&c2) {}", c1.gt(&c2));
    println!("c1 >= c2   {}", c1 >= c2);
    println!("c1.ge(&c2) {}", c1.ge(&c2));

    println!("c2 < c3    {}", c2 < c3);
    println!("c2 > c3    {}", c2 > c3);
}

fn demo_partial_ord_implemented() {
    println!("\nImplemented PartialOrd");

    let a1 = Angle { degrees: 10 };
    let a2 = Angle { degrees: 400 };

    println!("a1 < a2    {}", a1 < a2);
    println!("a1.lt(&a2) {}", a1.lt(&a2));
    println!("a1 <= a2   {}", a1 <= a2);
    println!("a1.le(&a2) {}", a1.le(&a2));

    println!("a1 > a2    {}", a1 > a2);
    println!("a1.gt(&a2) {}", a1.gt(&a2));
    println!("a1 >= a2   {}", a1 >= a2);
    println!("a1.ge(&a2) {}", a1.ge(&a2));
}

fn demo_partial_equality() {
    let p1 = PointV2 { x: 10, y: 20 };
    let p2 = PointV2 { x: 10, y: 20 };
    let p3 = PointV2 { x: 30, y: 40 };

    println!("p1 == p2   {}", p1 == p2);
    println!("p1.eq(&p2) {}", p1.eq(&p2));
    println!("p1 != p3   {}", p1 != p3);
    println!("p1.ne(&p3) {}", p1.ne(&p3));
}

fn demo_no_partial_equality() {
    let p1 = PointV1 { x: 10, y: 20 };
    let p2 = PointV1 { x: 10, y: 20 };
    let p3 = PointV1 { x: 30, y: 40 };

    // These statements won't work
    // println!("p1 == p2   {}", p1 == p2);
    // println!("p1.eq(&p2) {}", p1.eq(&p2));
    // println!("p1 != p3   {}", p1 != p3);
    // println!("p1.ne(&p3) {}", p1.ne(&p3));
}

fn display_array<T: std::fmt::Debug>(a: &[T]) {
    for elem in a {
        println!("{:?} ", elem);
    }
}

fn process_array_ints(arr: &[i32]) {
    println!("{} elements, {} bytes each", arr.len(), std::mem::size_of::<i32>());
}

fn process_array_floats(arr: &[f64]) {
    println!("{} elements, {} bytes each", arr.len(), std::mem::size_of::<f64>());
}

fn process_array<T>(arr: &[T]) {
    println!("{} elements, {} bytes each", arr.len(), std::mem::size_of::<T>());
}