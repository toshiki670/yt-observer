use yew::prelude::*;

use crate::route::Router;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Router/>
    }
}
