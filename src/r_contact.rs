use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Global;
use crate::w_nav::Nav;

struct Local {
    value: i64
}

#[function_component(Contact)]
pub fn view() -> Html {

    let (global, dispatch): (Rc<Global>, Dispatch<Global>) = use_store::<Global>();
    let local: UseStateHandle<Local> = use_state(|| Local {value: 0});
    
    let onclick_local: Callback<MouseEvent> = {
        let local_0: UseStateHandle<Local>= local.clone();   
        Callback::from(move |_| {
            local_0.set(Local {
                value: local_0.value + 1
            });
        })
    };
    let onclick_global: Callback<MouseEvent> = dispatch.reduce_mut_callback(|global| global.value += 1);

    
    html! {
        <>
            <Nav />
            <p>{"Contact"}</p><br />
            <button onclick={onclick_local}>{"local +1"}</button>
            <p>{format!("local {}", local.value)}</p>
            <button onclick={onclick_global}>{"global +1"}</button>
            <p>{format!("global {}",  global.value)}</p>
        </>
    }
}