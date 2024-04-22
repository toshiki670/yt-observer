use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1><Link<Route> to={Route::Secure}>{ "click here to go source" }</Link<Route>></h1> },
        Route::Secure => html! { <h1><Link<Route> to={Route::Home}>{ "click here to go Home" }</Link<Route>></h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
       <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
