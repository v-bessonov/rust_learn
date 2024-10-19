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

use numeric::*;
use simple_types::*;
use additional_techniques::*;
use control_flow::*;
use enums::{demo_simple_enums, demo_enum_with_data, demo_using_option_enum, demo_using_result_enum};
use arrays::*;
use tuples::*;
use vectors::demo_vectors;
use maps::demo_maps;
use scope::*;
use parts::Parts;
use strings::*;

fn main() {
    let course = Parts::Part1;

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
}



