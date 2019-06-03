#[macro_use] extern crate lambda_runtime as lambda;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

mod chrono;
mod serde;

use reqwest::Client;
use ::chrono::{DateTime, Utc, Datelike, Weekday};
use crate::chrono::LastDayOfMonth;
use std::collections::HashMap;
use lettre::{SmtpClient, Transport};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;
use tera::{Tera, Context};
use lettre_email::EmailBuilder;
use std::thread;
use arc_guard::ArcGuard;
use lambda_runtime::error::HandlerError;
use std::error::Error;
use serde_json::Value;
use std::str;

const METADATA_URL: &str = "https://wizzair.com/static/metadata.json";
const SEARCH_TIMETABLE_ENDPOINT: &str = "/search/timetable";
const TEMPLATE: &'static str = include_str!("../templates/index.html");

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(run);
    Ok(())
}

fn run(_: Value, _: lambda::Context) -> Result<(), HandlerError> {
    let metadata = reqwest::get(METADATA_URL)
        .expect("Failed to fetch the current metadata.")
        .text()
        .expect("Failed to deserialize the metadata.");

    let mut metadata = metadata.chars();

    // https://tools.ietf.org/html/rfc7159#section-8.1
    // Implementations MUST NOT add a byte order mark to the beginning
    //
    // But hey, who follows standards?
    metadata.next();

    let metadata: Metadata = serde_json::from_str(metadata.as_str()).unwrap();
    let search_timetable_url = metadata.api_url + SEARCH_TIMETABLE_ENDPOINT;

    let matched_flights_per_month = ArcGuard::new(HashMap::new());

    let current_time = Utc::now();

    let mut threads = vec![];
    //date is gonna fail after 12
    for i in 2..=4 {
        let i = i.clone();
        let search_timetable_url = search_timetable_url.clone();
        let current_time = current_time.clone();
        let matched_flights_per_month = matched_flights_per_month.clone();

        threads.push(thread::spawn(move|| {
            let month = current_time.with_day(1).unwrap().with_month(current_time.month() + i).unwrap();
            let from = format!("{}-{}-01", month.year(), month.month());
            let to = format!("{}-{}-{}", month.year(), month.month(), month.last_day_of_month());

            let mut flights: Flights = Client::new().post(&search_timetable_url)
                .body(json!({
                "adultCount": 1,
                "childCount": 0,
                "infantCount": 0,
                "priceType": "wdc",
                "flightList": [
                    {
                        "departureStation": "LTN",
                        "arrivalStation": "BEG",
                        "from": from,
                        "to": to
                    },
                    {
                        "departureStation": "BEG",
                        "arrivalStation": "LTN",
                        "from": from,
                        "to": to
                    }
                ]
            }).to_string())
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .expect("Failed to fetch flights.")
                .json()
                .expect("Failed to deserialize flights.");

            let mut matched_flights = Vec::new();

            for outbound_flight in flights.outbound_flights {
                match outbound_flight.departure_date.weekday() {
                    Weekday::Tue | Weekday::Sun => continue,
                    _ => ()
                }

                for (index, return_flight) in flights.return_flights.iter().enumerate() {
                    if let Weekday::Sat = return_flight.departure_date.weekday() {
                        continue;
                    }

                    let difference_in_days = return_flight
                        .departure_date
                        .signed_duration_since(outbound_flight.departure_date)
                        .num_days();

                    if difference_in_days >= 2 && difference_in_days < 4 {
                        matched_flights.push(FlightMatch{outbound_flight, return_flight: flights.return_flights.swap_remove(index)});
                        break;
                    }
                }
            }

            let matched_flights_per_month = matched_flights_per_month.arc();
            let mut matched_flights_per_month = matched_flights_per_month.lock().expect("Unable to lock Matched Flights Per Month.");
            matched_flights_per_month.insert(month.format("%B").to_string(), matched_flights);
        }));
    }

    for thread in threads {
        // Wait for the thread to finish. Returns a result.
        let _ = thread.join();
    }

    let mut mailer = SmtpClient::new_simple(env!("SMTP_HOST")).unwrap()
        .credentials(
            Credentials::new(
                env!("SMTP_USERNAME").to_string(),
                env!("SMTP_PASSWORD").to_string()
            )
        )
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Plain)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).transport();

    let mut context = Context::new();
    let matched_flights_per_month = matched_flights_per_month.arc();
    context.insert("matched_flights_per_month", &*matched_flights_per_month.lock().expect("Unable to lock Matched Flights Per Month."));

    let email = EmailBuilder::new()
        .from("noreply@wizzair-flight-finder.rs".to_string())
        .to("kunicmarko20@gmail.com".to_string())
        .subject("Wizzair Flight Finder")
        .html(Tera::one_off(TEMPLATE, &context, true).unwrap())
        .build()
        .expect("Unable to build email.");

    mailer.send(email.into()).unwrap();

    Ok(())
}

#[derive(Deserialize, Debug)]
struct Metadata {
    #[serde(rename = "apiUrl")]
    api_url: String,
}

#[derive(Deserialize, Debug)]
struct Flights {
    #[serde(rename = "outboundFlights")]
    outbound_flights: Vec<Flight>,
    #[serde(rename = "returnFlights")]
    return_flights: Vec<Flight>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Flight {
    #[serde(rename = "departureStation")]
    departure_station: String,
    #[serde(with = "serde::departure_dates", rename = "departureDates")]
    departure_date: DateTime<Utc>,
    price: Price,
}

#[derive(Serialize, Deserialize, Debug)]
struct Price {
    amount: f64,
}

#[derive(Serialize, Debug)]
struct FlightMatch {
    outbound_flight: Flight,
    return_flight: Flight,
}
