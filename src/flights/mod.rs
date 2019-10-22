use yew::prelude::*;
use super::Model;

pub fn view_flight(flight: Flight) -> Html<Model> {
  html!{
    <div class="flight-container">
      <div class="flight-info">
        <div class="flight-info-time"> { flight.departure.time.stamp } </div>
        <div class="flight-info-code"> { flight.departure.airportCode } </div>
        <div class="arrow">{ " > " }</div>
        <div class="flight-info-time"> { flight.arrival.time.stamp } </div>
        <div class="flight-info-code"> { flight.arrival.airportCode } </div>
      </div>
      <div class="flight-duration">{flight.flightDuration}</div>
      <div class="flight-price">{flight.cabins[0].displayPrice}</div>
    </div>
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flights {
  pub data: FlightInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlightInfo {
  pub flights: Vec<Flight>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flight {
  pub flightCode: String,
  pub flightDuration: String,
  pub arrival: FlightInstance,
  pub departure: FlightInstance,
  pub cabins: Vec<Cabin>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlightInstance {
  pub airportCode:	String,
  pub airportName:	String,
  pub cityCode:	String,
  pub cityName:	String,
  pub time: Time,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Time {
  pub stamp:	String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cabin {
  pub displayPrice:	f32,
}
