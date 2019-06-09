use tera::{Tera, Context};
use std::collections::HashMap;
use crate::flight_matcher::FlightMatch;

const TEMPLATE: &str = include_str!("../templates/index.html");

pub struct Renderer;

impl Renderer {
    pub fn render(matched_flights_per_month: HashMap<String, Vec<FlightMatch>>) -> String {
        let mut context = Context::new();
        context.insert("matched_flights_per_month", &matched_flights_per_month);
        Tera::one_off(TEMPLATE, &context, true).expect("Unable to render template.")
    }
}