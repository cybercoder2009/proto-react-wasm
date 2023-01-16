use yew::prelude::*;

#[function_component(Footer)]
pub fn view() -> Html {
    html! {
        <div id="footer">
            <ul>
                <li><a href="https://tiktok.com" target="_blank">{"Tiktok"}</a></li>
                <li><a href="https://twitter.com" target="_blank">{"Twitter"}</a></li>
            </ul>
        </div>
    }
}