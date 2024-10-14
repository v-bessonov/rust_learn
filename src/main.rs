mod numeric;
mod simple_types;
mod additional_techniques;
mod control_flow;
mod enums;

use numeric::*;
use simple_types::*;
use additional_techniques::*;
use control_flow::*;
use enums::{demo_simple_enums, demo_enum_with_data, demo_using_option_enum};

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
}



