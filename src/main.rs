#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

mod chrono;
mod serde;

use reqwest::Client;
use ::chrono::{DateTime, Utc, Datelike, Weekday};
use crate::chrono::LastDayOfMonth;
use std::collections::HashMap;

const METADATA_URL: &str = "https://wizzair.com/static/metadata.json";
const SEARCH_TIMETABLE_ENDPOINT: &str = "/search/timetable";

fn main() {
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
    let search_timetable_url = &(metadata.api_url + SEARCH_TIMETABLE_ENDPOINT);

    let mut matched_flights_per_month: HashMap<u32, Vec<FlightMatch>> = HashMap::new();

    for i in 2..=4 {
        let current_time = Utc::now();
        let current_time = current_time.with_month(current_time.month() + i).unwrap();
        let from = format!("{}-{}-01", current_time.year(), current_time.month());
        let to = format!("{}-{}-{}", current_time.year(), current_time.month(), current_time.last_day_of_month());

        let mut flights: Flights = Client::new().post(search_timetable_url)
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
                        "departureStation": "LTN",
                        "arrivalStation": "BEG",
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
                    matched_flights.push(FlightMatch(outbound_flight, flights.return_flights.swap_remove(index)));
                    break;
                }
            }
        }

        matched_flights_per_month.insert(i, matched_flights);
    }
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

#[derive(Deserialize, Debug)]
struct Flight {
    #[serde(rename = "departureStation")]
    departure_station: String,
    #[serde(rename = "arrivalStation")]
    arrival_station: String,
    #[serde(with = "serde::departure_dates", rename = "departureDates")]
    departure_date: DateTime<Utc>,
    price: Price,
}

impl Flight {
    fn price(&self) -> f64 {
        self.price.amount
    }
}

#[derive(Deserialize, Debug)]
struct Price {
    amount: f64,
}

#[derive(Debug)]
struct FlightMatch(Flight, Flight);
