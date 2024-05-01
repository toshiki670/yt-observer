use yew::prelude::*;
use yew_nested_router::prelude::{Router as YewRouter, *};

use crate::page::Page;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum Route {
    #[default]
    Dashboard,
    Channel,
    NotFound,
}

fn render(routes: Route) -> Html {
    match routes {
        Route::Dashboard => {
            html! {<Page><h1>{ "Dashboard" }</h1></Page> }
        }
        Route::Channel => {
            html! { <Page><h1>{ "Channel" }</h1></Page> }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Router)]
pub fn router() -> Html {

    html! {
        <YewRouter<Route> default={Route::Dashboard}>
            <Switch<Route> render={render} />
        </YewRouter<Route>>
    }
}





// #[function_component(Router)]
// pub fn router() -> Html {
//     let sidebar = html_nested! {
//         <PageSidebar>
//             <Nav>
//                 <NavList>
//                     <NavRouterItem<Route> to={Route::Index}>{"Index"}</NavRouterItem<Route>>
//                 </NavList>

//             </Nav>
//         </PageSidebar>
//     };
//     html! {
//         <BrowserRouter>
//             <Page sidebar>
//                 <PageSection>
//                     // <Switch<Route> render={switch} />
//                         {"my content"}
//                 </PageSection>

//             </Page>

//         </BrowserRouter>
//     }
// }

// #[function_component(Sidebar)]
// pub fn sidebar() -> Html {
//     html! {<PageSidebar><Nav></Nav></PageSidebar>}
// }
