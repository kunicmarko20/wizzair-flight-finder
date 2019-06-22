use ::chrono::{Utc, Datelike};
use std::collections::HashMap;
use crate::chrono::LastDayOfMonth;
use crate::thread_pool::ThreadPool;
use crate::reqwest::FlightsClient;
use crate::flight_matcher::{FlightMatcher, FlightMatch};
use crate::mailer::Mailer;
use crate::renderer::Renderer;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use chrono::DateTime;
use std::env;

pub fn run() {
    let mut thread_pool = ThreadPool::default();
    let (sender, receiver): (Sender<(String, Vec<FlightMatch>)>, Receiver<(String, Vec<FlightMatch>)>) = mpsc::channel();

    let start_month = env::var("START_MONTH").unwrap().parse::<u32>().expect("Not a number.");
    let end_month = start_month + env::var("NUMBER_OF_MONTHS").unwrap().parse::<u32>().expect("Not a number.");

    for i in start_month..=end_month {
        let i = i.clone();
        let sender = sender.clone();

        thread_pool.spawn(move|| {
            let search_month = search_month(i);

            let from = format!("{}-{}-01", search_month.year(), search_month.month());
            let to = format!("{}-{}-{}", search_month.year(), search_month.month(), search_month.last_day_of_month());

            let flights = FlightsClient::flights(from, to);

            sender.send(
                (search_month.format("%B").to_string(),
                    FlightMatcher::match_flights(flights))
            ).expect("Unable to send flight match.");
        });
    }

    let mut matched_flights_per_month = HashMap::new();

    for _ in start_month..=end_month {
        let matched_flights = receiver.recv().expect("Unable to receive flight match.");
        matched_flights_per_month.insert(matched_flights.0, matched_flights.1);
    }

    thread_pool.wait();

    let mut mailer = Mailer::new();

    mailer.send(Renderer::render(matched_flights_per_month));
}

fn search_month(add_months: u32) -> DateTime<Utc> {
    let now = Utc::now();

    let mut search_month = now.month() + add_months;

    //we only have 12 months
    if search_month > 12 {
        search_month -= 12;
    }

    now.with_day(1)
        .expect("Unable to set day of month.")
        .with_month(search_month)
        .expect("Unable to set month.")
}
