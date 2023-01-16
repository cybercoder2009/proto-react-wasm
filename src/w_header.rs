use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Header)]
pub fn view() -> Html {
    html! {
        <div id="header">
            <div>{"Brand"}</div>
            <ul>
                <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                <li><Link<Route> to={Route::Contact}>{"Contact"}</Link<Route>></li>
            </ul>
        </div>
    }
}