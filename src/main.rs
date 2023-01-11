mod store;
mod w_nav;
mod r_home;
mod r_contact;

use yew::prelude::*;
use yew_router::prelude::*;
use r_home::Home;
use r_contact::Contact;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <Home /> },
        Route::Contact => html!{ <Contact /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}