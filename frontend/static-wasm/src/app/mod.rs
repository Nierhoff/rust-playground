//! Routes by yew_router

use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::about::About;
use crate::pages::blog::Blog;
use crate::pages::careers::Careers;
use crate::pages::contact::Contact;
use crate::pages::home::Home;

use crate::pages::services::Services;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/careers")]
    Careers,
    #[at("/contact")]
    Contact,
    #[at("/blog")]
    Blog,
    #[at("/services")]
    Services,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<Home />},
        AppRoute::About => html! {<About />},
        AppRoute::Contact => html! {<Contact />},
        AppRoute::Careers => html! {<Careers />},
        AppRoute::Services => html! {<Services />},
        AppRoute::Blog => html! {<Blog />},
        AppRoute::NotFound => html! { "Page not found" },
    }
}

/// The root app component
#[function_component]
pub fn App() -> Html {
    html! {
        <HashRouter>
            <Switch<AppRoute> render={switch} />
        </HashRouter>
    }
}
