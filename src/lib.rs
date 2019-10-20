#![recursion_limit="256"]
extern crate yew;
extern crate stdweb;

use yew::prelude::*;
use yew::services::{
    ConsoleService,
    fetch::{FetchService, FetchTask, Request, Response},
};
use yew::events::IKeyboardEvent;
use yew::format::{Text, Nothing};

pub struct Model {
  fetch_service: FetchService,
  console: ConsoleService,

  link: ComponentLink<Model>,
  fetching: bool,

  edit_value: String,
  value: String,
  ft: Option<FetchTask>,
  payload: Option<String>,
}

#[derive(Debug)]
pub enum Msg {
  Update(String),
  Submit,
  FetchReady(Text),
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
      value: "https://bff.latam.com/ws/proxy/booking-webapp-bff/v1/public/revenue/bestprices/oneway?departure=2019-11-19&origin=POA&destination=GRU&adult=1&cabin=Y&promoCode=&country=BR&language=pt".to_string(),
      link: link,
      payload: None,

      ft: None,
    }
  }

  fn mounted(&mut self) -> ShouldRender {
        self.fetch_data();

        true
    }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    use Msg::*;
    match msg {
        Msg::Update(val) => {
            self.edit_value = val;
        }
        Msg::Submit => {
            self.value = self.edit_value.clone();
            self.fetch_data();
        }
        Msg::FetchReady(response) => {
            self.fetching = false;
            self.payload = response.map(|data| data).ok();
        }
        Msg::Ignore => {
            return false;
        }
        Msg::Nope => {}
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
            <section class="container">
                { self.view_data() }
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
        if let Some(value) = &self.payload {
            html! {
                { value }
            }
        } else {
            html! {
              <div class="loading">
                { "Loading..." }
              </div>
            }
        }
    }

  
    fn fetch_data(&mut self) {
        self.fetching = true;

        let callback = self.link.send_back(
            move |response: Response<Text>| {
                let (meta, data) = response.into_parts();

                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    Msg::Ignore
                }
            },
        );

        let request = Request::builder()
            .method("GET")
            .uri(self.value.clone())
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
