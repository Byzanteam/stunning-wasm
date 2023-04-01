#![cfg_attr(not(test), no_main)]

use jet_programmable_rust_binding::{outputs::Outputs, program, value_presenter::ValuePresenter};

fn entrypoint(_inputs: Vec<ValuePresenter>) -> Outputs {
    Outputs::build(vec![])
}

program!(entrypoint, vec![]);
