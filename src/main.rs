//#![allow(dead_code)]
//#![allow(unused_imports)]
mod numeric;
mod simple_types;
mod additional_techniques;
mod control_flow;
mod enums;
mod arrays;
mod tuples;
mod vectors;
mod maps;
mod scope;
mod parts;
mod strings;
mod copying_moving;
mod cloning;
mod borrowing;
mod functions;
mod closures;

use numeric::*;
use simple_types::*;
use additional_techniques::*;
use control_flow::*;
use enums::{demo_simple_enums, demo_enum_with_data, demo_using_option_enum, demo_using_result_enum};
use arrays::*;
use tuples::*;
use vectors::{demo_vectors, demo_closures_iteration};
use maps::demo_maps;
use scope::*;
use parts::Parts;
use strings::*;
use copying_moving::demo_copying_moving;
use cloning::demo_cloning;
use borrowing::*;
use functions::*;
use closures::*;

fn main() {
    let course = Parts::Part2;

    match course {
        Parts::Part1 => {
            run_part1();
        }
        Parts::Part2 => {
            run_part2();
        }
    }
}

fn run_part2() {
    demo_passing_value();
    demo_passing_references();
    demo_passing_mutable_references();
    demo_returning_value();
    demo_returning_reference();
    demo_returning_mutable_reference();
    demo_nested_functions();
    demo_closures();
    demo_closures_inferred_types();
    demo_closures_capture_reference();
    demo_closures_capture_value();
    demo_closures_iteration();
}

fn run_part1() {
    demo_integers();
    demo_floats();
    demo_other_simple_types();
    demo_additional_techniques();
    demo_if();
    demo_match();
    demo_loops();
    demo_break_continue();
    demo_simple_enums();
    demo_enum_with_data();
    demo_using_option_enum();
    demo_using_result_enum();
    demo_arrays();
    demo_arrays_techniques();
    demo_tuples();
    demo_vectors();
    demo_maps();
    demo_locals();
    demo_static_local();
    demo_static_global();

    println!("main, GLOBAL MESSAGE: {}", GLOBAL_MESSAGE);

    demo_static_mutable();
    demo_string_handling();
    demo_copying_moving();
    demo_cloning();
    demo_simple_borrowing();
    demo_borrow_checker();
    demo_string_slice_intro();
    demo_string_slice_techniques();
    demo_array_slice_intro();
    demo_array_slice_techniques();
}



