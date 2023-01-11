use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Global;
use crate::w_nav::Nav;

#[function_component(Contact)]
pub fn view() -> Html {

    let (state_global, dispatch): (Rc<Global>, Dispatch<Global>) = use_store::<Global>();

    html! {
        <>
            <Nav />
            <p>{ "Contact" }</p>
            <p>{ format!("Global {}",  state_global.value) }</p>
        </>
    }
}