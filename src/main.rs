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
mod structs;
mod mytypes;
mod structs_functionality;
mod mytraits;
mod mystructs;
mod traits;
mod generics;
mod concurrency;

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
use structs::*;
use structs_functionality::*;
use traits::*;
use generics::*;
use concurrency::*;

fn main() {
    let course = Parts::Part2Concurrency;

    match course {
        Parts::Part1 => {
            run_part1();
        }
        Parts::Part2 => {
            run_part2();
        }
        Parts::Part2Traits => {
            run_part2_traits();
        }
        Parts::Part2Generics => {
            run_part2_generics();
        }
        Parts::Part2Concurrency => {
            run_part2_concurrency();
        }

    }
}

fn run_part2_concurrency() {
    // demo_spawning_threads();
    // demo_joining_thread_single();
    // demo_joining_thread_multiple();
    // demo_capturing_state_implicit_move();
    // demo_capturing_state_explicit_move();
    // demo_channels_single_message();
    demo_channels_multiple_messages();
}

fn run_part2_generics() {
    demo_generic_structs();
    demo_generic_functions();
    demo_type_constraints();
    demo_partialeq_derived();
    demo_partialeq_implemented();
    demo_partialeq_implemented_diff_types();
    demo_eq_hash();
    demo_partial_ord();
    demo_ord();
    demo_closures_fn_once();
    demo_closures_fn_mut();
    demo_closures_fn();
}

fn run_part2_traits() {
    demo_trait_essentials();
    demo_trait_techniques();
    demo_inheritance_polymorphism();
    demo_trait_inheritance();
    demo_displayable();
    demo_debuggable();
    demo_droppable();
    demo_cloneable();
    demo_copyable();
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
    demo_accessing_struct();
    demo_struct_instances();
    demo_struct_pass_value();
    demo_struct_pass_reference();
    demo_struct_return_value();
    demo_struct_return_reference();
    demo_simple_struct_implementation();
    demo_mutable_struct_implementation();
    demo_modular_code_struct_implementation();
    demo_associated_functions_struct_implementation();
    demo_associated_data_struct_implementation();
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



