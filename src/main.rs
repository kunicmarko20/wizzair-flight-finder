#[macro_use] extern crate serde_derive;

mod serde;

use reqwest::Client;
use chrono::{DateTime, Utc};

const METADATA_URL: &str = "https://wizzair.com/static/metadata.json";
const FARE_CHART_ENDPOINT: &str = "/asset/farechart";

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

    let mut flights: Flights = Client::new().post(&(metadata.api_url + FARE_CHART_ENDPOINT))
        .body(r#"{
            "isRescueFare": false,
            "adultCount": 1,
            "childCount": 0,
            "dayInterval": 3,
            "wdc": true,
            "flightList": [
                {
                    "departureStation": "LTN",
                    "arrivalStation": "BEG",
                    "date": "2019-06-13"
                },
                {
                    "departureStation": "BEG",
                    "arrivalStation": "LTN",
                    "date": "2019-06-16"
                }
            ]
        }"#)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .expect("Failed to fetch flights.")
        .json()
        .expect("Failed to deserialize flights.");

    flights.remove_invalid_flights();

    dbg!(flights);
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

impl Flights {
    const INVALID_FLIGHT_PRICE: f64 = 0.0;

    // Better return flights for all days
    // even if there are no flights on some days
    // but wait, lets put a price 0.0 there
    fn remove_invalid_flights(&mut self) {
        self.outbound_flights
            .retain(|flight| flight.price() != Flights::INVALID_FLIGHT_PRICE);

        self.return_flights
            .retain(|flight| flight.price() != Flights::INVALID_FLIGHT_PRICE);
    }
}

#[derive(Deserialize, Debug)]
struct Flight {
    #[serde(rename = "departureStation")]
    departure_station: String,
    #[serde(rename = "arrivalStation")]
    arrival_station: String,
    #[serde(with = "serde::date")]
    date: DateTime<Utc>,
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

