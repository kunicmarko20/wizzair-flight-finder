use chrono::{Weekday, Datelike};
use crate::reqwest::{Flights, Flight};

pub struct FlightMatcher;

impl FlightMatcher {
    pub fn match_flights(mut flights: Flights) -> Vec<FlightMatch> {
        let mut matched_flights = Vec::new();

        for outbound_flight in flights.outbound_flights {
            match outbound_flight.departure_date.weekday() {
                Weekday::Fri => (),
                _ => continue
            }

            for (index, return_flight) in flights.return_flights.iter().enumerate() {
                match outbound_flight.departure_date.weekday() {
                    Weekday::Mon => (),
                    _ => continue
                }

                let difference_in_days = return_flight
                    .departure_date
                    .signed_duration_since(outbound_flight.departure_date)
                    .num_days();

                if difference_in_days > 3 {
                    continue;
                }

                matched_flights.push(FlightMatch{outbound_flight, return_flight: flights.return_flights.swap_remove(index)});
                break;
            }
        }

        matched_flights
    }
}
#[derive(Serialize, Debug)]
pub struct FlightMatch {
    outbound_flight: Flight,
    return_flight: Flight,
}
