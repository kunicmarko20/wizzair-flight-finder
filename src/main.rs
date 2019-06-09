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

fn main() {
    flight_finder::run();
}
