extern crate recommendations_paso2;
extern crate yew;

use recommendations_paso2::Model;
use yew::prelude::App;

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}