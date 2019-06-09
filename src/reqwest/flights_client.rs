use chrono::{DateTime, Utc};
use reqwest::Client;
use crate::reqwest::MetadataClient;
use crate::serde::departure_dates;

pub struct FlightsClient;

const SEARCH_TIMETABLE_PATH: &str = "/search/timetable";

lazy_static!{
    static ref SEARCH_TIMETABLE_URL: String = MetadataClient::api_url() + SEARCH_TIMETABLE_PATH;
}

impl FlightsClient {
    pub fn flights(from: String, to: String) -> Flights {
        Client::new().post(SEARCH_TIMETABLE_URL.as_str())
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
            .expect("Failed to deserialize flights.")
    }
}

#[derive(Deserialize, Debug)]
pub struct Flights {
    #[serde(rename = "outboundFlights")]
    pub outbound_flights: Vec<Flight>,
    #[serde(rename = "returnFlights")]
    pub return_flights: Vec<Flight>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flight {
    #[serde(rename = "departureStation")]
    departure_station: String,
    #[serde(with = "departure_dates", rename = "departureDates")]
    pub departure_date: DateTime<Utc>,
    price: Price,
}

#[derive(Serialize, Deserialize, Debug)]
struct Price {
    amount: f64,
}
