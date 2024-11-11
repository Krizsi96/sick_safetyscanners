use crate::data_output::application_data::input::Inputs;
use crate::data_output::application_data::output::Outputs;

mod input;
mod output;

const NUMBER_OF_MONITORING_CASES: usize = 20;
const NUMBER_OF_CUT_OFF_PATHS: usize = 20;

#[derive(PartialEq, Debug)]
struct ApplicationData {
    inputs: Inputs,
    outputs: Outputs,
}

impl ApplicationData {
    fn from_bytes(bytes: &[u8]) -> Self {
        todo!()
    }
}
