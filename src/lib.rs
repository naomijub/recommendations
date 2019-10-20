#![recursion_limit="128"]
extern crate yew;
extern crate stdweb;
#[macro_use] extern crate failure;

use yew::prelude::*;
use yew::services::FetchService;
use yew::format::{Json, Nothing};
use yew::services::fetch::{Request, Response};
use failure::Error;

pub struct Model {
  payload: String,
}

#[derive(Debug, Clone)]
pub enum Msg {
  Payload(String),
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model { payload: String::default() }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    use Msg::*;
    match msg {
      Payload(payload) => {
        if payload != self.payload {
            self.payload = payload;
            true
        } else {
            false
        }
      }
    }
  }
}

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
        <div class="lafran",>
            <div class="header",>{"Lafran"}</div>
            <div class="body",>
              <button onclick=|_| Msg::Payload("update".to_string())>
                    { "Get the payload!" }
              </button>
              <p style="font-family: 'Monaco', monospace;">
                  { self.payload.as_str() }
              </p>
            </div>
        </div>
    }
  }
}