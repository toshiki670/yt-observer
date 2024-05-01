use patternfly_yew::prelude::{Page as PatternflyYewPage, *};
use yew::prelude::*;

use crate::route::Route;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PageProps {
    pub children: Children,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    let sidebar = html_nested! {
        <PageSidebar>
            <Nav>
                <NavList>
                    <NavRouterItem<Route> to={Route::Dashboard}>{"Index"}</NavRouterItem<Route>>
                    <NavRouterItem<Route> to={Route::Channel}>{"Counter"}</NavRouterItem<Route>>
                </NavList>
            </Nav>
        </PageSidebar>
    };

    let brand = html! (
        <MastheadBrand>
            <Brand src="assets/images/pf-logo.svg" alt="Patternfly Logo" style="--pf-v5-c-brand--Height: 36px;"/>
        </MastheadBrand>
    );


    let tools = html!(
        <Toolbar full_height=true>
            <ToolbarContent>
                <ToolbarGroup
                    modifiers={ToolbarElementModifier::Right.all()}
                    variant={GroupVariant::IconButton}
                >
                </ToolbarGroup>
            </ToolbarContent>
        </Toolbar>
    );


    html! (
        <PatternflyYewPage {brand} {sidebar} {tools}>
            { for props.children.iter() }
        </PatternflyYewPage>
    )
}
