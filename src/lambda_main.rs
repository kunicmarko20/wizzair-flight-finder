#[macro_use] extern crate lambda_runtime as lambda;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;

pub mod chrono;
mod flight_finder;
pub mod flight_matcher;
pub mod mailer;
mod serde;
pub mod renderer;
pub mod reqwest;
pub mod thread_pool;

use std::error::Error;
use lambda_runtime::error::HandlerError;
use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(run);
    Ok(())
}

pub fn run(_: Value, _: lambda::Context) -> Result<(), HandlerError> {
    flight_finder::run();
    Ok(())
}
