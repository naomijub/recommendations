#![recursion_limit="256"]
extern crate yew;
extern crate stdweb;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod bestprices;
mod flights;
use bestprices::{BestPrices, view_itinerary, view_bestprices};
use flights::{Flights, view_flight};

use yew::prelude::*;
use yew::services::{
    ConsoleService,
    fetch::{FetchService, FetchTask, Request, Response},
};
// use yew::events::IKeyboardEvent;
use yew::format::{Text, Nothing};

pub struct Model {
  fetch_service: FetchService,
  console: ConsoleService,

  link: ComponentLink<Model>,
  fetching: bool,

  edit_value: String,
  value_bp: String,
  value_flights: String,
  ft: Option<FetchTask>,
  payload_bp: Option<BestPrices>,
  payload_flights: Option<Flights>
}

#[derive(Debug)]
pub enum Msg {
  Update(String),
  Submit,
  FetchBpReady(Text),
  FetchFlightsReady(Text),
  Ignore,
  Nope,
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Model {
      fetch_service: FetchService::new(),
      console: ConsoleService::new(),

      fetching: false,
      edit_value: "".to_string(),
      value_bp: "https://bff.latam.com/ws/proxy/booking-webapp-bff/v1/public/revenue/bestprices/oneway?departure=2019-11-19&origin=POA&destination=GRU&adult=1&cabin=Y&promoCode=&country=BR&language=pt".to_string(),
      value_flights: "https://bff.latam.com/ws/proxy/booking-webapp-bff/v1/public/revenue/recommendations/oneway?departure=2019-11-20&origin=POA&destination=GRU&adult=1&cabin=Y&country=BR&language=PT&home=pt_br&promoCode=".to_string(),
      link: link,
      payload_bp: None,
      payload_flights: None,

      ft: None,
    }
  }

  fn mounted(&mut self) -> ShouldRender {
        self.fetch_flights_data();

        true
    }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    use Msg::*;
    match msg {
        Update(val) => {
            self.edit_value = val;
        }
        Submit => {
            self.value_bp = self.edit_value.clone();
            self.fetch_bp_data();
        }
        FetchBpReady(response) => {
            self.payload_bp = Some(
              serde_json::from_str(&response.map(|data| data).unwrap()).unwrap()
            );
        },
        FetchFlightsReady(response) => {
          self.fetching = false;
          self.payload_flights = Some(
              serde_json::from_str(&response.map(|data| data).unwrap()).unwrap()
            );
          self.fetch_bp_data();
        },
        Ignore => {
            return false;
        }
        Nope => {}
      }
      true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let mut subnav_class = "sub theme-black border-white center ".to_string();

        if self.fetching {
            subnav_class.push_str("x-display");
        } else {
            subnav_class.push_str("inherit-display");
        }

        html! {
          <div class="lafran">
            <div class="header",>{"Lafran"}</div>
            { self.view_data() }
            <section class="container">
              {self.view_flights()}
            </section>
            <footer class="container" >
              <div class="flight-disclaimer">
              <h2><span>{"Informações gerais de voos"}</span></h2>
              <ul>
                <li><i class="icon-int-circle"></i><span>{"O modelo do avião pode variar por motivos operacionais."}</span></li>    
                <li><i class="icon-int-circle"></i><span>{"Os lugares e valores estão sujeitos à confirmação nas etapas seguintes."}</span></li>
                <li><i class="icon-int-circle"></i><span>{"Para alteração, será cobrada a taxa do perfil de tarifa escolhido ou 100% do valor da tarifa, o que for menor (mais diferença tarifária, se houver)."}</span></li>
                <li><i class="icon-int-circle"></i><span>{"Você pode pedir o reembolso integral da sua passagem desde que a solicitação seja feita até 24 horas após o recebimento do comprovante e que a sua compra tenha ocorrido ao menos 7 dias antes da data do voo, conforme art. 11 da Resolução nº 400/2016 da ANAC."}</span></li>
                </ul></div>
            </footer>
          </div>
        }
    }
}

impl Model {
    fn view_data(&self) -> Html<Model> {
        if let Some(value) = &self.payload_bp {
            let mut_value =  value.clone();
            html! {
              <div>
                { view_itinerary(mut_value.itinerary) }
                { view_bestprices(mut_value.bestPrices) }
              </div>
            }
        } else {
            html! {
              <div/>
            }
        }
    }

    fn view_flights(&self) -> Html<Model> {
        if let Some(value) = &self.payload_flights {
            let mut_value =  value.clone();
            html! {
              <div class="flight-top">
                { mut_value.data.flights.into_iter()
                .map(|flight| view_flight(flight))
                .collect::<Html<Model>>() }
              </div>
            }
        } else {
            html! {
              <div class="loading">
                { "Loading..." }
              </div>
            }
        }
    }

  
    fn fetch_bp_data(&mut self) {
        self.fetching = true;

        let callback = self.link.send_back(
            move |response: Response<Text>| {
                let (meta, data) = response.into_parts();

                if meta.status.is_success() {
                    Msg::FetchBpReady(data)
                } else {
                    Msg::Ignore
                }
            },
        );

        let request = Request::builder()
            .method("GET")
            .uri(self.value_bp.clone())
            .body(Nothing)
            .unwrap();

        let task = self.fetch_service.fetch(request, callback);
        self.ft = Some(task);
    }

    fn fetch_flights_data(&mut self) {
      self.fetching = true;

        let callback = self.link.send_back(
            move |response: Response<Text>| {
                let (meta, data) = response.into_parts();

                if meta.status.is_success() {
                    Msg::FetchFlightsReady(data)
                } else {
                    Msg::Ignore
                }
            },
        );

        let request = Request::builder()
            .method("GET")
            .uri(self.value_flights.clone())
            .body(Nothing)
            .unwrap();

        let task = self.fetch_service.fetch(request, callback);
        self.ft = Some(task);
    }
}

// <input
//     type="text",
//     autocomplete="off",
//     disabled=self.fetching,
//     value=&self.value,
//     oninput=|e| Msg::Update(e.value),
//     onkeypress=|e| {
//         if e.key() == "Enter" { Msg::Submit } else { Msg::Nope }
//     },
// />

