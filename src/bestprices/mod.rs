use yew::prelude::*;
use super::Model;

pub fn view_itinerary(itinerary: Itinerary) -> Html<Model> {
  html!{
    <div class="itinerary">
      <div class="route"> {itinerary.clone()} </div>
      <div class="date"> {itinerary.date} </div>
    </div>
  }
}

pub fn view_bestprices(best_prices: Vec<BestPrice>) -> Html<Model> {
  html!{
    <div class="container">
      <div class="bestprice-title"> { "Escolha seu voo" } </div>
      <div class="bestprices"> {
        best_prices.into_iter().map(|best|
          html!{
            <div class="bestprice-container">
              <div class="bestprice-date"> {best.date} </div>
              <div class="bestprice-price">
                {best.price} 
              </div>
            </div>
          }
        ).collect::<Html<Model>>()
      } </div>
    </div>
  }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BestPrices {
  pub itinerary: Itinerary,
  pub bestPrices: Vec<BestPrice>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Itinerary {
  pub date: String,
  pub originDestinations: Vec<OriginDestination>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OriginDestination {
  pub duration: i32,
  pub departure: Target,
  pub arrival: Target,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Target {
  pub airport: String,
  pub city: String,
  pub country: String,
  pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BestPrice {
  pub date: String,
  pub available: bool,
  pub price: Price,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Price {
  pub amount: f32,
  pub currency: String
}

impl std::fmt::Display for Itinerary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} a {}", self.originDestinations.first().unwrap().departure.city,
          self.originDestinations.last().unwrap().arrival.city)
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.currency, self.amount)
    }
}
