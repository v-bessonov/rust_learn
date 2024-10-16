mod numeric;
mod simple_types;
mod additional_techniques;
mod control_flow;
mod enums;
mod arrays;
mod tuples;
mod vectors;
mod maps;

use numeric::*;
use simple_types::*;
use additional_techniques::*;
use control_flow::*;
use enums::{demo_simple_enums, demo_enum_with_data, demo_using_option_enum, demo_using_result_enum};
use arrays::*;
use tuples::*;
use vectors::demo_vectors;
use maps::demo_maps;

fn main() {
    println!("Hello, world!");
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
}



