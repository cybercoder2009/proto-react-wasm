use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Nav)]
pub fn view() -> Html {
    html! {
        <>
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <span>{"  / "}</span>
            <Link<Route> to={Route::Contact}>{"Contact"}</Link<Route>>
        </>
    }
}