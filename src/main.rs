use github_actions::set_outputs;
use handler::handler;

use crate::github_actions::get_input;
use crate::handler::HandlerParams;

mod github_actions;
mod handler;

#[derive(Debug)]
pub struct Inputs {
    pub example_input: Option<String>,
}

impl From<Inputs> for HandlerParams {
    fn from(inputs: Inputs) -> Self {
        Self {
            example_input: inputs.example_input.expect("example-input is required"),
        }
    }
}

fn main() {
    // Take inputs from environment variables.
    let inputs = Inputs {
        example_input: get_input("example-input"),
    };
    // TODO: this as a core.debug statement?
    println!("{:?}", inputs);
    let result = handler(inputs);
    set_outputs(result.unwrap());
}
