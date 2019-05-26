#[macro_use] extern crate serde_derive;

mod serde;

use reqwest::Client;
use chrono::{DateTime, Utc, Datelike, Weekday};

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

    let flights: Flights = Client::new().post(&(metadata.api_url + SEARCH_TIMETABLE_ENDPOINT))
        .body(r#"{
            "adultCount": 1,
            "childCount": 0,
            "infantCount": 0,
            "priceType": "wdc",
            "flightList": [
                {
                    "departureStation": "LTN",
                    "arrivalStation": "BEG",
                    "from": "2019-06-01",
                    "to": "2019-06-30"
                },
                {
                    "departureStation": "LTN",
                    "arrivalStation": "BEG",
                    "from": "2019-06-01",
                    "to": "2019-06-30"
                }
            ]
        }"#)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .expect("Failed to fetch flights.")
        .json()
        .expect("Failed to deserialize flights.");

    let mut matched_flights = Vec::new();

    for outbound_flight in &flights.outbound_flights {
        match outbound_flight.departure_date.weekday() {
            Weekday::Tue | Weekday::Sun => continue,
            _ => ()
        }

        for return_flight in &flights.return_flights {
            if let Weekday::Sat = return_flight.departure_date.weekday() {
                continue;
            }

            let difference_in_days = return_flight
                .departure_date
                .signed_duration_since(outbound_flight.departure_date)
                .num_days();

            if difference_in_days >= 2 && difference_in_days < 4 {
                matched_flights.push(FlightMatch(outbound_flight, return_flight));
            }
        }
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
struct FlightMatch<'a>(&'a Flight, &'a Flight);

trait ExpandDateTime {
    fn is_leap_year(&self) -> bool;

    fn last_day_of_month(&self) -> u32;
}

impl ExpandDateTime for DateTime<Utc> {
    fn is_leap_year(&self) -> bool {
        self.naive_utc().year() % 4 == 0 && (self.naive_utc().year() % 100 != 0 || self.naive_utc().year() % 400 == 0)
    }

    fn last_day_of_month(&self) -> u32 {
        match self.naive_utc().month() {
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap_year() {
                29
            } else {
                28
            },
            _ => 31,
        }
    }
}